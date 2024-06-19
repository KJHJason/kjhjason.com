use crate::constants::constants;
use crate::middleware::csrf;
use actix_web::dev::ServiceRequest;
use actix_web::http::Method;
use actix_web::{HttpMessage, HttpRequest};
use rand::Rng as _;

/// This assumes that the environment variables are in hex format
pub fn get_bytes_from_env(key: &str) -> Vec<u8> {
    let hex = std::env::var(key).unwrap();
    hex::decode(hex).unwrap()
}

pub fn get_default_secret_key() -> Vec<u8> {
    get_bytes_from_env(constants::SECRET_KEY)
}

pub fn get_default_salt() -> Vec<u8> {
    get_bytes_from_env(constants::SECRET_KEY_SALT)
}

pub fn get_default_key_info(salt: Vec<u8>, info: Vec<u8>) -> hmac_serialiser_rs::KeyInfo {
    hmac_serialiser_rs::KeyInfo {
        key: get_default_secret_key(),
        salt,
        info,
    }
}

// https://rust-random.github.io/book/guide-rngs.html
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut random_bytes = vec![0u8; length];
    rand::thread_rng().fill(&mut random_bytes[..]);
    random_bytes
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

pub fn get_csrf_token(req: &HttpRequest) -> String {
    req.extensions()
        .get::<csrf::CsrfValue>()
        .unwrap()
        .get_csrf_token()
}

pub fn is_logged_in(req: &HttpRequest) -> bool {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => true,
        None => false,
    }
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
    let csrf_header_json = format!(r#"{{"{}":"{}"}}"#, constants::CSRF_HEADER_NAME, &csrf_value);
    TemplateValues {
        nonce,
        csrf_header: constants::CSRF_HEADER_NAME.to_string(),
        csrf_value,
        csrf_header_json,
        is_logged_in: is_logged_in(req),
    }
}
