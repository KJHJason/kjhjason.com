use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::{header, header::HeaderValue};
use actix_web::Error;
use futures_util::future::LocalBoxFuture;
use regex::Regex;
use std::future::ready;
use std::rc::Rc;

#[derive(Clone)]
pub struct CacheStrictPathValue {
    pub path: String,
    pub value: String,
}

#[derive(Clone)]
pub struct CachePathValue {
    pub path: Regex,
    pub value: String,
}

#[derive(Clone)]
pub struct CachePaths {
    pub strict_paths: Vec<CacheStrictPathValue>,
    pub regex_paths: Vec<CachePathValue>,
}

#[derive(Clone)]
pub struct CacheControlMiddleware {
    inner: CachePaths,
}

impl CacheControlMiddleware {
    pub fn new(cache_paths: CachePaths) -> Self {
        Self { inner: cache_paths }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CacheControlMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CacheControlMiddlewareService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CacheControlMiddlewareService {
            service,
            inner: Rc::new(self.inner.clone()),
        }))
    }
}

pub struct CacheControlMiddlewareService<S> {
    service: S,
    inner: Rc<CachePaths>,
}

impl<S, B> Service<ServiceRequest> for CacheControlMiddlewareService<S>
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
        let path = req.path();
        let mut cache_control = String::new();
        for cache in &self.inner.strict_paths {
            if cache.path == path {
                cache_control = cache.value.clone();
                break;
            }
        }
        if cache_control.is_empty() {
            for cache in &self.inner.regex_paths {
                if cache.path.is_match(path) {
                    cache_control = cache.value.clone();
                    break;
                }
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            if !cache_control.is_empty() {
                res.headers_mut().insert(
                    header::CACHE_CONTROL,
                    HeaderValue::from_str(&cache_control).unwrap(),
                );
            }
            Ok(res)
        })
    }
}
