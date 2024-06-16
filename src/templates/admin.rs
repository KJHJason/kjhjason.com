use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "admin/new_blog.html")]
pub struct NewBlog {
    pub common: TemplateValues,
}

#[derive(Template)]
#[template(path = "components/unlocked.html")]
pub struct Unlocked;

#[derive(Template)]
#[template(path = "components/locked.html")]
pub struct Locked;
