use crate::constants::constants;
use crate::model::csrf;
use crate::utils::security;
use actix_web::cookie::{time as cookie_time, Cookie, SameSite};
use base64::{engine::general_purpose, Engine as _};
use hmac_serialiser_rs::SignerLogic;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CsrfToken {
    token: String,

    #[serde(with = "chrono::serde::ts_seconds")]
    expiry: chrono::DateTime<chrono::Utc>,
}

impl CsrfToken {
    fn new(token: String, expiry: chrono::DateTime<chrono::Utc>) -> CsrfToken {
        CsrfToken { token, expiry }
    }
}

impl hmac_serialiser_rs::Data for CsrfToken {
    fn get_exp(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        Some(self.expiry)
    }
}

#[derive(Clone)]
pub struct CsrfSigner {
    cookie_name: String,
    header_name: String,
    token_len: usize,
    signer: hmac_serialiser_rs::HmacSigner,
}

impl Default for CsrfSigner {
    fn default() -> Self {
        Self::new(
            constants::CSRF_COOKIE_NAME,
            constants::CSRF_HEADER_NAME,
            constants::CSRF_TOKEN_LENGTH,
            security::get_default_key_info(
                security::get_bytes_from_env(constants::CSRF_KEY_SALT),
                vec![],
            ),
            hmac_serialiser_rs::algorithm::Algorithm::SHA1,
            hmac_serialiser_rs::Encoder::UrlSafeNoPadding,
        )
    }
}

impl CsrfSigner {
    pub fn new(
        cookie_name: &str,
        header_name: &str,
        token_len: usize,
        key_info: hmac_serialiser_rs::KeyInfo,
        algo: hmac_serialiser_rs::algorithm::Algorithm,
        encoder: hmac_serialiser_rs::Encoder,
    ) -> CsrfSigner {
        Self {
            cookie_name: cookie_name.to_string(),
            header_name: header_name.to_string(),
            token_len,
            signer: hmac_serialiser_rs::HmacSigner::new(key_info, algo, encoder),
        }
    }

    // Cryptographically secure random token generator
    // Generates 32 random bytes base64-encoded string
    fn generate_csrf_token(&self) -> String {
        let random_bytes = security::generate_random_bytes(self.token_len);
        self.signer.sign(&CsrfToken::new(
            general_purpose::STANDARD_NO_PAD.encode(&random_bytes),
            chrono::Utc::now() + chrono::Duration::seconds(constants::CSRF_MAX_AGE),
        ))
    }

    pub fn create_csrf_cookie(&self) -> Cookie<'_> {
        let csrf_token = self.generate_csrf_token();
        Cookie::build(&self.cookie_name, csrf_token)
            .http_only(true)
            .domain(constants::get_domain())
            .same_site(SameSite::Lax)
            .path("/")
            .secure(!constants::DEBUG_MODE)
            .max_age(cookie_time::Duration::seconds(constants::CSRF_MAX_AGE))
            .finish()
    }

    fn verify_token(&self, csrf_token: &str) -> Result<CsrfToken, csrf::CsrfError> {
        self.signer
            .unsign::<CsrfToken>(csrf_token)
            .map_err(|_| csrf::CsrfError::InvalidToken)
    }

    pub fn extract_csrf_cookie(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<String, csrf::CsrfError> {
        let csrf_cookie = match req.cookie(&self.cookie_name) {
            Some(cookie) => cookie.value().to_string(),
            None => return Err(csrf::CsrfError::MissingToken),
        };

        let csrf_token = self.verify_token(&csrf_cookie)?;
        Ok(csrf_token.token)
    }

    pub fn extract_csrf_header(
        &self,
        req: &actix_web::dev::ServiceRequest,
    ) -> Result<String, csrf::CsrfError> {
        let csrf_header = match req.headers().get(&self.header_name) {
            Some(header) => header.to_str().unwrap().to_string(),
            None => return Err(csrf::CsrfError::MissingToken),
        };

        let csrf_token = self.verify_token(&csrf_header)?;
        Ok(csrf_token.token)
    }

    pub fn get_csrf_cookie_name(&self) -> &str {
        &self.cookie_name
    }
}
