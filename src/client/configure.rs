use crate::client::admin::new_blog;
use crate::client::auth::{login_admin, login_auth, login_redirect};
use crate::client::general::{blog, blog_id, experiences, index, projects, skills};
use actix_web::web;

pub fn add_client_routes(cfg: &mut web::ServiceConfig) {
    add_general_routes(cfg);
    add_auth_routes(cfg);
    add_admin_routes(cfg);
}

fn add_general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(experiences)
        .service(projects)
        .service(skills)
        .service(blog)
        .service(blog_id);
}

fn add_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login_redirect)
        .service(login_admin)
        .service(login_auth);
}

fn add_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(new_blog);
}
