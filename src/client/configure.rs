use crate::client::admin::{edit_blog, new_blog, profile};
use crate::client::auth::{login_admin, login_auth, login_redirect};
use crate::client::general::{awards, blog_id, blogs, certificates, experiences, index, projects, resume, skills, testimonials};
use actix_web::web;

#[inline]
pub fn add_client_routes(cfg: &mut web::ServiceConfig) {
    add_general_routes(cfg);
    add_auth_routes(cfg);
    add_admin_routes(cfg);
}

#[inline]
fn add_general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(resume)
        .service(experiences)
        .service(testimonials)
        .service(projects)
        .service(skills)
        .service(certificates)
        .service(awards)
        .service(blogs)
        .service(blog_id);
}

#[inline]
fn add_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login_redirect)
        .service(login_admin)
        .service(login_auth);
}

#[inline]
fn add_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(new_blog).service(edit_blog).service(profile);
}
