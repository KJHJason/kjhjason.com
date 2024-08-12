use crate::api::admin::{
    delete_blog, new_blog, preview_blog, publish_blog_post, unpublish_blog_post, update_blog,
    upload_blog_files,
};
use crate::api::admin_profile::{change_password, generate_2fa, remove_2fa, setup_2fa};
use crate::api::auth::{admin_honeypot, login, logout};
use crate::api::csrf::get_csrf_token;
use crate::api::general::{api_health, api_index};
use crate::constants;

use actix_web::web;

#[inline]
pub fn add_api_routes(cfg: &mut web::ServiceConfig) {
    add_admin_routes(cfg);
    add_admin_profile_routes(cfg);
    add_auth_routes(cfg);
    add_general_routes(cfg);
}

#[inline]
fn add_admin_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(delete_blog)
        .service(preview_blog)
        .service(new_blog)
        .service(publish_blog_post)
        .service(unpublish_blog_post)
        .service(update_blog)
        .service(upload_blog_files);
}

#[inline]
fn add_admin_profile_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(generate_2fa)
        .service(setup_2fa)
        .service(remove_2fa)
        .service(change_password);
}

#[inline]
fn add_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(admin_honeypot)
        .route(
            &format!("/api{}", constants::get_login_uri_path()),
            web::post().to(login),
        )
        .service(logout);
}

#[inline]
fn add_general_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(api_index)
        .service(api_health)
        .service(get_csrf_token);
}
