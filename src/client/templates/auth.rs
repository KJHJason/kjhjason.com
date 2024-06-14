use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "auth/login.html")]
pub struct Login<'a> {
    pub common: TemplateValues,
    pub login_url: &'a str,
}
