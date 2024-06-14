use crate::model::blog::BlogIdentifier;
use crate::utils::security::{extract_for_template, TemplateValues};
use actix_web::{get, web::Path, HttpRequest, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "general/index.html")]
struct Index {
    common: TemplateValues,
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    Index {
        common: extract_for_template(&req),
    }
}

#[derive(Template)]
#[template(path = "general/experiences.html")]
struct Experiences {
    common: TemplateValues,
}

#[get("/experiences")]
async fn experiences(req: HttpRequest) -> impl Responder {
    Experiences {
        common: extract_for_template(&req),
    }
}

#[derive(Template)]
#[template(path = "general/projects.html")]
struct Projects {
    common: TemplateValues,
}

#[get("/projects")]
async fn projects(req: HttpRequest) -> impl Responder {
    Projects {
        common: extract_for_template(&req),
    }
}

#[derive(Template)]
#[template(path = "general/skills.html")]
struct Skills {
    common: TemplateValues,
}

#[get("/skills")]
async fn skills(req: HttpRequest) -> impl Responder {
    Skills {
        common: extract_for_template(&req),
    }
}

#[derive(Template)]
#[template(path = "general/blog.html")]
struct Blog {
    common: TemplateValues,
}

#[get("/blog")]
async fn blog(req: HttpRequest) -> impl Responder {
    Blog {
        common: extract_for_template(&req),
    }
}

#[get("/blog/{id}")]
async fn blog_id(req: HttpRequest, blog_id: Path<BlogIdentifier>) -> impl Responder {
    println!("{}", blog_id.to_string());
    Blog {
        common: extract_for_template(&req),
    }
}
