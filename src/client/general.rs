use crate::client::templates::general::{Blog, Experiences, Index, Projects, Skills};
use crate::model::blog::BlogIdentifier;
use crate::utils::security::extract_for_template;
use actix_web::{get, web::Path, HttpRequest, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    Index {
        common: extract_for_template(&req),
    }
}

#[get("/experiences")]
async fn experiences(req: HttpRequest) -> impl Responder {
    Experiences {
        common: extract_for_template(&req),
    }
}

#[get("/projects")]
async fn projects(req: HttpRequest) -> impl Responder {
    Projects {
        common: extract_for_template(&req),
    }
}

#[get("/skills")]
async fn skills(req: HttpRequest) -> impl Responder {
    Skills {
        common: extract_for_template(&req),
    }
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
