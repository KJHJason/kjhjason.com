use crate::utils::security::{convert_vec_str_to_owned, generate_random_bytes, is_protected};

use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::HeaderValue;
use actix_web::http::{header, Method};
use actix_web::{Error, HttpMessage};
use base64::{engine::general_purpose, Engine as _};
use futures_util::future::LocalBoxFuture;
use std::future::ready;
use std::rc::Rc;

#[derive(Clone)]
pub struct ContentSecurityPolicies<'a> {
    pub script_src: Vec<&'a str>,
    pub style_src: Vec<&'a str>,
    pub frame_src: Vec<&'a str>,
    pub default_src: Vec<&'a str>,
    pub base_uri: Vec<&'a str>,
    pub img_src: Vec<&'a str>,
    pub font_src: Vec<&'a str>,
    pub object_src: Vec<&'a str>,
    pub form_action: Vec<&'a str>,
    pub frame_ancestors: Vec<&'a str>,
}

impl Default for ContentSecurityPolicies<'_> {
    fn default() -> Self {
        Self {
            script_src: vec!["'self'"],
            style_src: vec!["'self'", "https:", "'unsafe-inline'"],
            frame_src: vec!["'self'"],
            default_src: vec!["'self'"],
            base_uri: vec!["'self'"],
            img_src: vec!["'self'", "data:"],
            font_src: vec!["'self'", "https:", "data:"],
            object_src: vec!["'none'"],
            form_action: vec!["'self'"],
            frame_ancestors: vec!["'self'"],
        }
    }
}

fn generate_nonce(n_bytes: usize) -> String {
    let nonce = generate_random_bytes(n_bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(&nonce)
}

fn add_csp_header(csp_option: &str, csp_value: &Vec<&str>) -> Option<String> {
    if csp_value.is_empty() {
        return None;
    }
    let csp_value = csp_value.join(" ");
    Some(format!("{} {}", csp_option, csp_value))
}

fn build_csp_header(csp: &ContentSecurityPolicies, nonce: &str) -> String {
    let directives = vec![
        add_csp_header("default-src", &csp.default_src),
        add_csp_header(&format!("script-src 'nonce-{}'", nonce), &csp.script_src),
        add_csp_header(&format!("style-src 'nonce-{}'", nonce), &csp.style_src),
        add_csp_header("frame-src", &csp.frame_src),
        add_csp_header("base-uri", &csp.base_uri),
        add_csp_header("img-src", &csp.img_src),
        add_csp_header("font-src", &csp.font_src),
        add_csp_header("object-src", &csp.object_src),
        add_csp_header("form-action", &csp.form_action),
        add_csp_header("frame-ancestors", &csp.frame_ancestors),
    ];

    let mut csp_header = String::new();
    for directive in directives {
        if let Some(directive) = directive {
            csp_header.push_str(format!("{}; ", directive).as_str());
        }
    }
    csp_header
}

pub struct CspNonce {
    nonce: String,
}

impl Default for CspNonce {
    // Mainly in the event that the CspNonce is not set.
    // which usually happens on errors for
    // whitelisted routes like 404 in the static routes.
    fn default() -> Self {
        Self {
            nonce: "".to_string(),
        }
    }
}

impl CspNonce {
    fn new(nonce: &str) -> Self {
        Self {
            nonce: nonce.to_string(),
        }
    }
    pub fn get_nonce(&self) -> &str {
        &self.nonce
    }
}

#[derive(Clone)]
struct CspMiddlewareConfig {
    nonce_len: usize,
    whitelist: Vec<(Method, String)>,
    whitelist_regex: Vec<(Method, regex::Regex)>,
    csp_config: ContentSecurityPolicies<'static>,
}

impl CspMiddlewareConfig {
    pub fn new(
        nonce_len: usize,
        whitelist: Vec<(Method, &str)>,
        whitelist_regex: Vec<(Method, regex::Regex)>,
        csp_config: ContentSecurityPolicies<'static>,
    ) -> Self {
        Self {
            nonce_len,
            whitelist: convert_vec_str_to_owned(whitelist),
            whitelist_regex,
            csp_config,
        }
    }
}

impl Default for CspMiddlewareConfig {
    fn default() -> Self {
        Self {
            nonce_len: 16, // 128 bits
            whitelist: vec![],
            whitelist_regex: vec![],
            csp_config: ContentSecurityPolicies::default(),
        }
    }
}

#[derive(Clone)]
pub struct CspMiddleware {
    inner: CspMiddlewareConfig,
}

impl CspMiddleware {
    pub fn new(
        nonce_len: usize,
        whitelist: Vec<(Method, &str)>,
        whitelist_regex: Vec<(Method, regex::Regex)>,
        csp_options: ContentSecurityPolicies<'static>,
    ) -> Self {
        Self {
            inner: CspMiddlewareConfig::new(nonce_len, whitelist, whitelist_regex, csp_options),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for CspMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CspMiddlewareService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CspMiddlewareService {
            service,
            inner: Rc::new(self.inner.clone()),
        }))
    }
}

pub struct CspMiddlewareService<S> {
    service: S,
    inner: Rc<CspMiddlewareConfig>,
}

impl<S, B> Service<ServiceRequest> for CspMiddlewareService<S>
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
        let mut csp_response_header = String::new();
        let is_protected = is_protected(&self.inner.whitelist, &self.inner.whitelist_regex, &req);
        if is_protected {
            let nonce = generate_nonce(self.inner.nonce_len);
            req.extensions_mut().insert(CspNonce::new(&nonce));
            csp_response_header = build_csp_header(&self.inner.csp_config, &nonce);
        }

        let csp_response_header = csp_response_header;
        let fut = self.service.call(req);
        Box::pin(async move {
            let mut res = fut.await?;
            if is_protected {
                res.headers_mut().insert(
                    header::CONTENT_SECURITY_POLICY,
                    HeaderValue::from_str(&csp_response_header).unwrap(),
                );
            }
            Ok(res)
        })
    }
}
