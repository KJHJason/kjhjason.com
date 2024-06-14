use crate::utils::security::{extract_for_template, TemplateValues};
use actix_web::{get, web, HttpRequest, Responder};
use askama_actix::Template;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct Login {
    common: TemplateValues,
    login_url: String,
}

#[get("/login")]
async fn login_redirect() -> web::Redirect {
    web::Redirect::to("/admin").temporary()
}

#[get("/admin")]
async fn login_admin(req: HttpRequest) -> impl Responder {
    Login {
        common: extract_for_template(&req),
        login_url: "/api/admin".to_string(),
    }
}

#[get("/auth/login")]
async fn login_auth(req: HttpRequest) -> impl Responder {
    Login {
        common: extract_for_template(&req),
        login_url: "/api/auth/login".to_string(),
    }
}
