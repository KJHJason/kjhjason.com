use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "general/index.html")]
pub struct Index {
    pub common: TemplateValues,
}

#[derive(Template)]
#[template(path = "general/experiences.html")]
pub struct Experiences {
    pub common: TemplateValues,
}

// title, img, img_alt, desc, tags, link
pub struct ProjectInfo<'a> {
    pub title: &'a str,
    pub img: &'a str,
    pub img_alt: &'a str,
    pub desc: &'a str,
    pub tags: Vec<&'a str>,
    pub link: &'a str,
    pub date: &'a str,
}

#[derive(Template)]
#[template(path = "general/projects.html")]
pub struct Projects<'a> {
    pub common: TemplateValues,
    pub projects: Vec<ProjectInfo<'a>>,
}

#[derive(Template)]
#[template(path = "general/skills.html")]
pub struct Skills {
    pub common: TemplateValues,
}

pub struct BlogPostInfo {
    pub id: String,
    pub title: String,
    pub date: String,
    pub views: i64,
    pub tags: Vec<String>,
}

#[derive(Template)]
#[template(path = "general/blog.html")]
pub struct Blog {
    pub common: TemplateValues,
    pub blogs: Vec<BlogPostInfo>,
}

#[derive(Template)]
#[template(path = "general/blog_post.html")]
pub struct BlogPost<'a> {
    pub common: TemplateValues,
    pub id: &'a str,
    pub title: &'a str,
    pub date: &'a str,
    pub last_modified: &'a str,
    pub readable_date: &'a str,
    pub views: i64,
    pub content: &'a str,
    pub public: bool,
    pub tags: &'a Vec<String>,
}
