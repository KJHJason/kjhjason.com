use crate::models::checkbox;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_res: String,
    pub remember: Option<checkbox::State>,
}

impl LoginData {
    pub fn remember_session(&self) -> bool {
        if self.remember.is_none() {
            return false;
        }
        self.remember.as_ref().unwrap().get_state()
    }
}
