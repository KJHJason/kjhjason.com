use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct Login {
    pub common: TemplateValues,
    pub login_url: String,
}
