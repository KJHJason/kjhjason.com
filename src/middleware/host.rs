use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use actix_web::{Error, HttpResponse};
use futures_util::future::LocalBoxFuture;
use std::future::ready;

#[derive(Clone)]
pub struct HostMiddleware;

impl<S, B> Transform<S, ServiceRequest> for HostMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Transform = HostMiddlewareService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(HostMiddlewareService { service }))
    }
}

pub struct HostMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for HostMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B, BoxBody>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let host = match req.headers().get(header::HOST) {
            Some(host) => host.to_str().unwrap_or("unknown"),
            None => "unknown",
        };
        if host == "kjhjason-com.fly.dev" {
            // return redirect to https://kjhjason.com
            return Box::pin(async {
                let res = HttpResponse::MovedPermanently()
                    .append_header((header::LOCATION, "https://kjhjason.com"))
                    .finish();
                Ok(req.into_response(res).map_into_right_body())
            });
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res.map_into_left_body())
        })
    }
}
