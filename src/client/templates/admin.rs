use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "admin/new_blog.html")]
pub struct NewBlog {
    pub common: TemplateValues,
}
