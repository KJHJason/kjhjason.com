use serde::Deserialize;

#[derive(Deserialize)]
pub struct Setup2fa {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_res: String,
    pub secret: String,
    #[serde(rename = "totp-code")]
    pub totp_code: String,
    #[serde(rename = "current-password")]
    pub current_password: String,
}
