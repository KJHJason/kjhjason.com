use crate::constants::constants;
use crate::middleware::csrf;
use crate::utils::auth::is_logged_in;
use actix_web::dev::ServiceRequest;
use actix_web::http::Method;
use actix_web::{HttpMessage, HttpRequest};
use rand::Rng as _;

#[inline]
pub fn get_default_key_info(salt: Vec<u8>, info: Vec<u8>) -> hmac_serialiser::KeyInfo {
    hmac_serialiser::KeyInfo {
        key: constants::get_secret_key(),
        salt,
        info,
    }
}

// https://rust-random.github.io/book/guide-rngs.html
#[inline]
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut random_bytes = vec![0u8; length];
    rand::thread_rng().fill(&mut random_bytes[..]);
    random_bytes
}

#[inline]
pub fn convert_vec_str_to_owned(vec_str: Vec<(Method, &str)>) -> Vec<(Method, String)> {
    let mut vec_str_owned = Vec::with_capacity(vec_str.len());
    for (method, path) in vec_str {
        vec_str_owned.push((method, path.to_string()));
    }
    vec_str_owned
}

pub fn is_protected(
    whitelist: &[(Method, String)],
    whitelist_regex: &[(Method, regex::Regex)],
    req: &ServiceRequest,
) -> bool {
    let method = req.method();
    let path = req.path();

    if whitelist
        .iter()
        .any(|(allowed_method, allowed_path)| allowed_method == method && allowed_path == path)
    {
        return false;
    }
    if whitelist_regex
        .iter()
        .any(|(allowed_method, allowed_path)| {
            allowed_method == method && allowed_path.is_match(path)
        })
    {
        return false;
    }
    true
}

#[inline]
pub fn get_csrf_token(req: &HttpRequest) -> String {
    req.extensions()
        .get::<csrf::CsrfValue>()
        .map(|csrf_value| csrf_value.get_csrf_token())
        .unwrap_or_else(|| {
            log::warn!(
                "CSRF token not found in request extensions for {}",
                req.path()
            );
            "".to_string()
        })
}

#[inline]
pub fn get_csrf_header_json(req: &HttpRequest, csrf_token: Option<&str>) -> String {
    let csrf_token = csrf_token
        .map(|csrf_token| csrf_token.to_string())
        .unwrap_or_else(|| get_csrf_token(req));
    format!(r#"{{"{}":"{}"}}"#, constants::CSRF_HEADER_NAME, csrf_token,)
}

#[derive(Debug)]
pub struct TemplateValues {
    pub nonce: String,
    pub csrf_header: String,
    pub csrf_value: String,
    pub csrf_header_json: String,
    pub is_logged_in: bool,
}

pub fn extract_for_template(req: &HttpRequest) -> TemplateValues {
    let nonce = {
        let extensions = req.extensions();
        extensions
            .get::<crate::middleware::csp::CspNonce>()
            // usually happens on errors response for whitelisted routes like the static routes
            .unwrap_or(&crate::middleware::csp::CspNonce::default())
            .get_nonce()
            .to_string()
    };
    let csrf_value = get_csrf_token(req);
    let csrf_header_json = get_csrf_header_json(req, Some(csrf_value.as_str()));
    TemplateValues {
        nonce,
        csrf_header: constants::CSRF_HEADER_NAME.to_string(),
        csrf_value,
        csrf_header_json,
        is_logged_in: is_logged_in(req),
    }
}
