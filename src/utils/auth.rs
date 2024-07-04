use crate::middleware::auth::UserClaim;
use actix_web::{HttpMessage, HttpRequest};

pub mod cf_turnstile {
    /// Note: Remember to import the necessary modules for the macro to work
    ///
    /// ```rust
    /// use crate::security::cf_turnstile;
    /// use crate::errors::auth::AuthError;
    /// ```
    macro_rules! verify_captcha {
        ($req:expr, $cf_turnstile_res:expr) => {
            if !cf_turnstile::verify_request($req, $cf_turnstile_res).await {
                return Err(AuthError::CaptchaFailed);
            }
        };
    }

    pub(crate) use verify_captcha;
}

#[inline]
pub fn is_logged_in(req: &HttpRequest) -> bool {
    match req.extensions().get::<UserClaim>() {
        Some(_) => true,
        None => false,
    }
}
