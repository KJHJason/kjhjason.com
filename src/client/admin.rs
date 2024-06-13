use actix_web::{get, HttpRequest, Responder};
use askama_actix::Template;
use crate::utils::security::extract_for_template;

#[derive(Template)]
#[template(path = "admin/new_blog.html")]
struct NewBlog {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
}

#[get("/admin/new/blog")]
async fn new_blog(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    NewBlog {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
    }
}
