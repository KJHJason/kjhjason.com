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

#[derive(Template)]
#[template(path = "general/projects.html")]
pub struct Projects {
    pub common: TemplateValues,
}

#[derive(Template)]
#[template(path = "general/skills.html")]
pub struct Skills {
    pub common: TemplateValues,
}

#[derive(Template)]
#[template(path = "general/blog.html")]
pub struct Blog {
    pub common: TemplateValues,
}
