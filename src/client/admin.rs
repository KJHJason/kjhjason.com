use crate::utils::security::{extract_for_template, TemplateValues};
use actix_web::{get, HttpRequest, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "admin/new_blog.html")]
struct NewBlog {
    common: TemplateValues,
}

#[get("/admin/new/blog")]
async fn new_blog(req: HttpRequest) -> impl Responder {
    NewBlog {
        common: extract_for_template(&req),
    }
}
