use crate::model::blog::BlogIdentifier;
use crate::utils::security::extract_for_template;
use actix_web::{get, web::Path, HttpRequest, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "general/index.html")]
struct Index {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
}

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Index {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}

#[derive(Template)]
#[template(path = "general/experiences.html")]
struct Experiences {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
}

#[get("/experiences")]
async fn experiences(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Experiences {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}

#[derive(Template)]
#[template(path = "general/projects.html")]
struct Projects {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
}

#[get("/projects")]
async fn projects(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Projects {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}

#[derive(Template)]
#[template(path = "general/skills.html")]
struct Skills {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
}

#[get("/skills")]
async fn skills(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Skills {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}

#[derive(Template)]
#[template(path = "general/blog.html")]
struct Blog {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
}

#[get("/blog")]
async fn blog(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Blog {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}

#[get("/blog/{id}")]
async fn blog_id(req: HttpRequest, blog_id: Path<BlogIdentifier>) -> impl Responder {
    println!("{}", blog_id.to_string());
    let values = extract_for_template(&req);
    Blog {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}
