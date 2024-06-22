use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChangePassword {
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_res: String,
    #[serde(rename = "current-password")]
    pub current_password: String,
    #[serde(rename = "new-password")]
    pub new_password: String,
    #[serde(rename = "confirm-password")]
    pub confirm_password: String,
    #[serde(rename = "totp-input")]
    pub totp_input: Option<String>,
}
