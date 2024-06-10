use crate::constants::constants;
use crate::security::jwt;
use actix_web::cookie::{time as cookie_time, Cookie};
use base64::{engine::general_purpose, Engine as _};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CsrfToken {
    token: String,

    #[serde(with = "crate::utils::datetime::rfc3339")]
    expiry: chrono::DateTime<chrono::Utc>,
}

impl CsrfToken {
    fn new(token: String, expiry: chrono::DateTime<chrono::Utc>) -> CsrfToken {
        CsrfToken { token, expiry }
    }
}

impl jwt::Claim for CsrfToken {
    fn get_exp(&self) -> chrono::DateTime<chrono::Utc> {
        self.expiry
    }
}

// Cryptographically secure random token generator
// Generates 32 random bytes base64-encoded string
fn generate_csrf_token() -> String {
    let mut random_bytes = [0u8; constants::CSRF_TOKEN_LENGTH];
    thread_rng().fill(&mut random_bytes);

    jwt::sign(&CsrfToken::new(
        general_purpose::STANDARD.encode(&random_bytes),
        chrono::Utc::now() + chrono::Duration::seconds(constants::CSRF_MAX_AGE),
    ))
    .unwrap_or_else(|_| "".to_string())
}

pub fn create_csrf_cookie() -> Cookie<'static> {
    Cookie::build(constants::CSRF_COOKIE_NAME, generate_csrf_token())
        .http_only(false) // Allow JavaScript to read the cookie to put it in the header
        .domain(constants::get_domain())
        .path("/")
        .secure(!constants::DEBUG_MODE)
        .max_age(cookie_time::Duration::seconds(constants::CSRF_MAX_AGE))
        .finish()
}

pub fn get_csrf_cookie(req: &actix_web::HttpRequest) -> Option<String> {
    match req.cookie(constants::CSRF_COOKIE_NAME) {
        Some(cookie) => Some(cookie.value().to_string()),
        None => None,
    }
}

pub fn verify_csrf_token(req: &actix_web::HttpRequest) -> bool {
    let csrf_cookie = match req.cookie(constants::CSRF_COOKIE_NAME) {
        Some(cookie) => match jwt::unsign::<CsrfToken>(&cookie.value()) {
            Ok(token) => token,
            Err(_) => return false,
        },
        None => return false,
    };
    let csrf_header = match req.headers().get(constants::CSRF_HEADER_NAME) {
        Some(header) => header.to_str().unwrap_or_default().to_string(),
        None => return false,
    };
    csrf_cookie.token == csrf_header
}
