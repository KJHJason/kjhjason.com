use crate::client::templates::admin::NewBlog;
use crate::utils::security::extract_for_template;
use actix_web::{get, HttpRequest, Responder};

#[get("/admin/new/blog")]
async fn new_blog(req: HttpRequest) -> impl Responder {
    NewBlog {
        common: extract_for_template(&req),
    }
}
