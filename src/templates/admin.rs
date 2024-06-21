use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "admin/new_blog.html")]
pub struct NewBlog<'a> {
    pub common: TemplateValues,
    pub post_blog_btn_txt: &'a str,
}

#[derive(Template)]
#[template(path = "admin/edit_blog.html")]
pub struct EditBlog<'a> {
    pub common: TemplateValues,
    pub id: &'a str,
    pub title: &'a str,
    pub content: &'a str,
    pub public: bool,
    pub tags: &'a str,
    pub post_blog_btn_txt: &'a str,
}

#[derive(Template)]
#[template(path = "admin/profile.html")]
pub struct Profile {
    pub common: TemplateValues,
    pub has_2fa: bool,
}

#[derive(Template)]
#[template(path = "components/unlocked.html")]
pub struct Unlocked;

#[derive(Template)]
#[template(path = "components/locked.html")]
pub struct Locked;
