use crate::utils::security::extract_for_template;
use actix_web::{get, web, HttpRequest, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct Login {
    csrf_header: String,
    nonce: String,
    is_logged_in: bool,
    login_url: String,
}

#[get("/login")]
async fn login_redirect() -> web::Redirect {
    web::Redirect::to("/admin").temporary()
}

#[get("/admin")]
async fn login_admin(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Login {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
        login_url: "/api/admin".to_string(),
    }
}

#[get("/auth/login")]
async fn login_auth(req: HttpRequest) -> impl Responder {
    let values = extract_for_template(&req);
    Login {
        csrf_header: values.csrf_header,
        nonce: values.nonce,
        is_logged_in: values.is_logged_in,
        login_url: "/api/auth/login".to_string(),
    }
}
