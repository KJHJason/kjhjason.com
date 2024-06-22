use crate::utils::security::TemplateValues;
use askama::Template;

#[derive(Template)]
#[template(path = "general/index.html")]
pub struct Index {
    pub common: TemplateValues,
}

pub struct ExperienceInfo<'a> {
    pub time: &'a str,
    pub finished: bool,
    pub title: &'a str,
    pub sub_title: &'a str,
    pub desc: &'a str,
}

#[derive(Template)]
#[template(path = "general/experiences.html")]
pub struct Experiences<'a> {
    pub common: TemplateValues,
    pub experiences: Vec<ExperienceInfo<'a>>,
}

// title, img, img_alt, desc, tags, link
pub struct ProjectInfo<'a> {
    pub title: &'a str,
    pub img: &'a str,
    pub img_alt: &'a str,
    pub desc: &'a str,
    pub tags: Vec<&'a str>,
    pub link: &'a str,
    pub presentation_link: &'a str,
    pub date: &'a str,
}

#[derive(Template)]
#[template(path = "general/projects.html")]
pub struct Projects<'a> {
    pub common: TemplateValues,
    pub projects: Vec<ProjectInfo<'a>>,
}

pub struct SkillInfo<'a> {
    pub link: &'a str,
    pub img_src: &'a str,
    pub img_alt: &'a str,
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "general/skills.html")]
pub struct Skills<'a> {
    pub common: TemplateValues,
    pub languages: Vec<SkillInfo<'a>>,
    pub backend: Vec<SkillInfo<'a>>,
    pub frontend: Vec<SkillInfo<'a>>,
    pub database: Vec<SkillInfo<'a>>,
    pub deployment: Vec<SkillInfo<'a>>,
    pub general: Vec<SkillInfo<'a>>,
}

pub struct CertificateInfo<'a> {
    pub title: &'a str,
    pub issuer: &'a str,
    pub cred_id: &'a str,
    pub link: &'a str,
    pub date: &'a str,
    pub expiry: &'a str,
    pub img_src: &'a str,
    pub img_alt: &'a str,
}

#[derive(Template)]
#[template(path = "general/certificates.html")]
pub struct Certificates<'a> {
    pub common: TemplateValues,
    pub certificates: Vec<CertificateInfo<'a>>,
}

pub struct AwardInfo<'a> {
    pub title: &'a str,
    pub issuer: &'a str,
    pub file_url: &'a str,
    pub date: &'a str,
    pub img_src: &'a str,
    pub img_alt: &'a str,
}

#[derive(Template)]
#[template(path = "general/awards.html")]
pub struct Awards<'a> {
    pub common: TemplateValues,
    pub awards: Vec<AwardInfo<'a>>,
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
