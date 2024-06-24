use crate::client::admin::{edit_blog, new_blog, profile};
use crate::client::auth::{login_admin, login_auth, login_redirect};
use crate::client::general::{
    awards, blog_id, blogs, certificates, experiences, index, projects, resume, skills,
    testimonials,
};
use crate::client::static_files::{favicon, sweetalert_css, sweetalert_js};
use actix_web::web;

#[inline]
pub fn add_client_routes(cfg: &mut web::ServiceConfig) {
    add_general_routes(cfg);
    add_static_routes(cfg);
    add_auth_routes(cfg);
    add_admin_routes(cfg);
}

#[inline]
fn add_static_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(favicon)
        .service(sweetalert_js)
        .service(sweetalert_css)
        .service(
            // Note: due to the error middleware, the 404 html page will
            // be rendered instead of the default actix error text response
            // if the static path is not found. E.g. /static/test.png will
            // return the 404 html page instead of the default error text response.
            actix_files::Files::new("/static", "./static"),
        );
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
