use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{header, header::HeaderValue};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;
use std::future::ready;
use std::rc::Rc;

#[derive(Clone)]
pub struct HstsOptions {
    pub max_age: i64,
    pub include_subdomains: bool,
    pub preload: bool,
}

impl Default for HstsOptions {
    fn default() -> Self {
        Self {
            max_age: 60 * 60 * 24 * 365, // 1 year
            include_subdomains: true,
            preload: false, // IncludeSubDomain must be true for Preload to work
        }
    }
}

impl HstsOptions {
    pub fn construct_header(&self) -> Option<String> {
        if self.max_age == 0 {
            return None;
        }

        let mut header = format!("max-age={}", self.max_age);
        if self.include_subdomains {
            header.push_str("; includeSubDomains");
        }
        if self.preload {
            header.push_str("; preload");
        }
        Some(header)
    }
}

#[derive(Clone)]
pub struct HstsMiddleware {
    inner: HstsOptions,
}

impl HstsMiddleware {
    pub fn new(hsts_options: HstsOptions) -> Self {
        Self {
            inner: hsts_options,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for HstsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = HstsMiddlewareService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HstsMiddlewareService {
            service,
            inner: Rc::new(self.inner.clone()),
        }))
    }
}

pub struct HstsMiddlewareService<S> {
    service: S,
    inner: Rc<HstsOptions>,
}

impl<S, B> Service<ServiceRequest> for HstsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let hsts_header = self.inner.construct_header();
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            if let Some(hsts_header) = hsts_header {
                res.headers_mut().insert(
                    header::STRICT_TRANSPORT_SECURITY,
                    HeaderValue::from_str(&hsts_header).unwrap(),
                );
            }
            Ok(res)
        })
    }
}
