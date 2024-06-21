use crate::client::admin::{edit_blog, new_blog, profile};
use crate::client::auth::{login_admin, login_auth, login_redirect};
use crate::client::general::{
    awards, blog_id, blogs, certificates, experiences, index, projects, skills,
};
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
        .service(certificates)
        .service(awards)
        .service(blogs)
        .service(blog_id);
}

fn add_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login_redirect)
        .service(login_admin)
        .service(login_auth);
}

fn add_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(new_blog).service(edit_blog).service(profile);
}
