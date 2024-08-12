use crate::utils::security::TemplateValues;

use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub common: TemplateValues,
    pub status: u16,
    pub message: &'a str,
}
