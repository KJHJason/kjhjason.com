use crate::constants::constants;
use crate::security::csrf;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures::Future;
use std::pin::Pin;
use std::rc::Rc;

pub struct CsrfMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CsrfMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CsrfMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CsrfMiddlewareService {
            service: Rc::new(service),
        })
    }
}

pub struct CsrfMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for CsrfMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        if !req.cookie(constants::AUTH_COOKIE_NAME).is_some() {
            let csrf_cookie = csrf::create_csrf_cookie();
            if let Some(existing_cookies) = req.headers().get(actix_web::http::header::COOKIE) {
                let mut cookie_value = existing_cookies.to_str().unwrap().to_owned();
                cookie_value.push_str("; ");
                cookie_value.push_str(&csrf_cookie.to_string());
                req.headers_mut().insert(
                    actix_web::http::header::COOKIE,
                    cookie_value.parse().unwrap(),
                );
            } else {
                req.headers_mut().insert(
                    actix_web::http::header::SET_COOKIE,
                    csrf_cookie.to_string().parse().unwrap(),
                );
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
