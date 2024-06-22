use serde::Deserialize;

#[derive(Deserialize)]
pub struct Remove2fa {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_res: String,
    #[serde(rename = "current-password")]
    pub current_password: String,
}
