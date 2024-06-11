use crate::model::csrf as csrf_model;
use crate::security::csrf::CsrfSigner;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, Method};
use actix_web::Error;
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct CsrfMiddlewareConfig {
    csrf_signer: CsrfSigner,
    whitelist: Vec<(Method, String)>,
}

impl CsrfMiddlewareConfig {
    pub fn new(csrf_signer: CsrfSigner, whitelist: Vec<(Method, String)>) -> CsrfMiddlewareConfig {
        CsrfMiddlewareConfig {
            csrf_signer,
            whitelist,
        }
    }
    pub fn add_to_whitelist(&mut self, method: Method, path: String) {
        self.whitelist.push((method, path));
    }
    pub fn is_protected(&self, method: &Method, path: &str) -> bool {
        for (allowed_method, allowed_path) in &self.whitelist {
            if allowed_method == method && allowed_path == path {
                return false;
            }
        }
        true
    }
    pub fn get_csrf_cookie_name(&self) -> &str {
        self.csrf_signer.get_csrf_cookie_name()
    }
    pub fn get_csrf_cookie(&self, req: &ServiceRequest) -> Result<String, csrf_model::CsrfError> {
        self.csrf_signer.extract_csrf_cookie(req)
    }
    pub fn get_csrf_header(&self, req: &ServiceRequest) -> Result<String, csrf_model::CsrfError> {
        self.csrf_signer.extract_csrf_header(req)
    }
}

impl Default for CsrfMiddlewareConfig {
    fn default() -> Self {
        CsrfMiddlewareConfig {
            csrf_signer: CsrfSigner::default(),
            whitelist: vec![],
        }
    }
}

#[derive(Clone)]
pub struct CsrfMiddleware {
    config: Arc<Mutex<CsrfMiddlewareConfig>>,
}

impl CsrfMiddleware {
    pub fn new(signer: Option<CsrfSigner>, whitelist: Vec<(Method, String)>) -> CsrfMiddleware {
        let config = match signer {
            Some(signer) => CsrfMiddlewareConfig::new(signer, whitelist),
            None => {
                let mut config = CsrfMiddlewareConfig::default();
                for (method, path) in whitelist {
                    config.add_to_whitelist(method, path);
                }
                config
            }
        };
        CsrfMiddleware {
            config: Arc::new(Mutex::new(config)),
        }
    }
}

pub struct CsrfMiddlewareService<S> {
    service: Rc<S>,
    config: Arc<Mutex<CsrfMiddlewareConfig>>,
}

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
            config: self.config.clone(),
        })
    }
}

impl<S, B> Service<ServiceRequest> for CsrfMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if self
            .config
            .lock()
            .unwrap()
            .is_protected(req.method(), req.path())
        {
            let csrf_cookie = match self.config.lock().unwrap().get_csrf_cookie(&req) {
                Ok(csrf_cookie) => csrf_cookie,
                Err(e) => {
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized(e.to_string()))
                    });
                }
            };
            let csrf_header = match self.config.lock().unwrap().get_csrf_header(&req) {
                Ok(csrf_header) => csrf_header,
                Err(e) => {
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized(e.to_string()))
                    });
                }
            };
            if csrf_cookie != csrf_header {
                return Box::pin(async move {
                    Err(actix_web::error::ErrorUnauthorized(
                        csrf_model::CsrfError::InvalidToken.to_string(),
                    ))
                });
            }
        }

        let mut csrf_cookie = String::new();
        let req_has_csrf_cookie = req
            .cookie(self.config.lock().unwrap().get_csrf_cookie_name())
            .is_some();
        if !req_has_csrf_cookie {
            csrf_cookie = self
                .config
                .lock()
                .unwrap()
                .csrf_signer
                .create_csrf_cookie()
                .to_string();
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            if req_has_csrf_cookie {
                return Ok(res);
            }

            // Retrieve existing cookies from response headers
            let mut cookies = String::new();
            let existing_cookies = res.headers().get(header::SET_COOKIE);
            if let Some(existing_cookies) = existing_cookies {
                cookies = existing_cookies.to_str().unwrap().to_string();
            }

            // Add CSRF cookie to existing cookies if it doesn't exist
            cookies.push_str(&csrf_cookie);

            // Insert the new cookies into the response headers
            res.headers_mut()
                .insert(header::SET_COOKIE, HeaderValue::from_str(&cookies).unwrap());
            Ok(res)
        })
    }
}
