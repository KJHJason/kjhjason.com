use crate::constants::constants;
use crate::model::csrf;
use crate::security::jwt;
use crate::security::jwt::JwtSignerLogic;
use crate::utils::security;
use actix_web::cookie::{time as cookie_time, Cookie};
use base64::{engine::general_purpose, Engine as _};
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

#[derive(Clone)]
pub struct CsrfSigner {
    cookie_name: String,
    header_name: String,
    token_len: usize,
    jwt_signer: jwt::JwtSigner,
}

impl Default for CsrfSigner {
    fn default() -> Self {
        Self::new(
            constants::CSRF_COOKIE_NAME,
            constants::CSRF_HEADER_NAME,
            constants::CSRF_TOKEN_LENGTH,
            security::get_default_jwt_key(),
            jsonwebtoken::Algorithm::HS512,
        )
    }
}

impl CsrfSigner {
    pub fn new(
        cookie_name: &str,
        header_name: &str,
        token_len: usize,
        secret_key: Vec<u8>,
        algo: jsonwebtoken::Algorithm,
    ) -> CsrfSigner {
        Self {
            cookie_name: cookie_name.to_string(),
            header_name: header_name.to_string(),
            token_len,
            jwt_signer: jwt::JwtSigner::new(secret_key, algo),
        }
    }

    // Cryptographically secure random token generator
    // Generates 32 random bytes base64-encoded string
    fn generate_csrf_token(&self) -> String {
        let random_bytes = security::generate_random_bytes(self.token_len);
        self.jwt_signer
            .sign(&CsrfToken::new(
                general_purpose::STANDARD.encode(&random_bytes),
                chrono::Utc::now() + chrono::Duration::seconds(constants::CSRF_MAX_AGE),
            ))
            .unwrap_or_else(|_| "".to_string())
    }

    pub fn create_csrf_cookie(&self) -> Cookie<'_> {
        let csrf_token = self.generate_csrf_token();
        Cookie::build(&self.cookie_name, csrf_token)
            .http_only(false) // Allow JavaScript to read the cookie to put it in the header
            .domain(constants::get_domain())
            .path("/")
            .secure(!constants::DEBUG_MODE)
            .max_age(cookie_time::Duration::seconds(constants::CSRF_MAX_AGE))
            .finish()
    }

    pub fn extract_csrf_cookie(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<String, csrf::CsrfError> {
        req.cookie(&self.cookie_name)
            .map(|cookie| cookie.value().to_string())
            .ok_or(csrf::CsrfError::MissingToken)
    }

    pub fn extract_csrf_header(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<String, csrf::CsrfError> {
        req.headers()
            .get(&self.header_name)
            .map(|header| header.to_str().unwrap_or_default().to_string())
            .ok_or(csrf::CsrfError::MissingToken)
    }

    pub fn get_csrf_cookie_name(&self) -> &str {
        &self.cookie_name
    }
}
