use crate::model::csrf as csrf_model;
use crate::security::csrf::CsrfSigner;
use crate::utils::security::is_protected;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, Method};
use actix_web::{Error, HttpMessage};
use futures::future::{ok, Ready};
use futures::task::{Context, Poll};
use futures_util::future::LocalBoxFuture;
use std::rc::Rc;

pub struct HasCsrfCookie;

#[derive(Clone)]
struct CsrfMiddlewareConfig {
    csrf_signer: CsrfSigner,
    whitelist: Vec<(Method, String)>,
    whitelist_regex: Vec<(Method, regex::Regex)>,
}

impl CsrfMiddlewareConfig {
    pub fn new(
        csrf_signer: CsrfSigner,
        whitelist: Vec<(Method, String)>,
        whitelist_regex: Vec<(Method, regex::Regex)>,
    ) -> Self {
        Self {
            csrf_signer,
            whitelist,
            whitelist_regex,
        }
    }
    pub fn set_whitelist(&mut self, whitelist: Vec<(Method, String)>) {
        self.whitelist = whitelist;
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
        Self {
            csrf_signer: CsrfSigner::default(),
            whitelist: vec![],
            whitelist_regex: vec![],
        }
    }
}

#[derive(Clone)]
pub struct CsrfMiddleware {
    config: CsrfMiddlewareConfig,
}

impl CsrfMiddleware {
    pub fn new(
        signer: Option<CsrfSigner>,
        whitelist: Vec<(Method, String)>,
        whitelist_regex: Vec<(Method, regex::Regex)>,
    ) -> Self {
        let config = match signer {
            Some(signer) => CsrfMiddlewareConfig::new(signer, whitelist, whitelist_regex),
            None => {
                let mut config = CsrfMiddlewareConfig::default();
                config.set_whitelist(whitelist);
                config
            }
        };
        Self { config }
    }
}

pub struct CsrfMiddlewareService<S> {
    service: Rc<S>,
    inner: Rc<CsrfMiddlewareConfig>,
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
            inner: Rc::new(self.config.clone()),
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
        // Since rust do not natively support negative lookahead
        if req.path().starts_with("/api/")
            && is_protected(&self.inner.whitelist, &self.inner.whitelist_regex, &req)
        {
            let csrf_cookie = match self.inner.get_csrf_cookie(&req) {
                Ok(csrf_cookie) => csrf_cookie,
                Err(e) => {
                    log::warn!("CSRF cookie error: {}", e);
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized(e.to_string()))
                    });
                }
            };
            let csrf_header = match self.inner.get_csrf_header(&req) {
                Ok(csrf_header) => csrf_header,
                Err(e) => {
                    log::warn!("CSRF header error: {}", e);
                    return Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized(e.to_string()))
                    });
                }
            };
            if csrf_cookie != csrf_header {
                return Box::pin(async move {
                    log::warn!("CSRF token mismatch");
                    Err(actix_web::error::ErrorUnauthorized(
                        csrf_model::CsrfError::InvalidToken.to_string(),
                    ))
                });
            }
        }

        let mut csrf_cookie = String::new();
        let req_has_csrf_cookie = req.cookie(self.inner.get_csrf_cookie_name()).is_some();
        if !req_has_csrf_cookie {
            csrf_cookie = self.inner.csrf_signer.create_csrf_cookie().to_string();

            // inject the CSRF cookie into the request for the handler to check
            req.extensions_mut().insert(HasCsrfCookie);
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
