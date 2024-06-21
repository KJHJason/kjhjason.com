use serde::Deserialize;

#[derive(Deserialize)]
pub struct Setup2fa {
    pub secret: String,
    pub totp_code: String,
}
