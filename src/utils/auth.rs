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
