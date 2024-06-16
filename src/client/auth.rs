use crate::constants::constants;
use crate::templates::auth::Login;
use crate::utils::security::extract_for_template;
use actix_web::http::header::{ContentType, LOCATION};
use actix_web::{get, web, HttpRequest, HttpResponse};
use askama::Template;

#[get("/login")]
async fn login_redirect() -> web::Redirect {
    web::Redirect::to("/admin").temporary()
}

#[get("/admin")]
async fn login_admin(req: HttpRequest) -> HttpResponse {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish(),
        None => {
            let template = Login {
                common: extract_for_template(&req),
                login_url: "api/admin",
                client_login_url: "admin",
            };
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(template.render().unwrap())
        }
    }
}

#[get("/auth/login")]
async fn login_auth(req: HttpRequest) -> HttpResponse {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish(),
        None => {
            let template = Login {
                common: extract_for_template(&req),
                login_url: "api/auth/login",
                client_login_url: "auth/login",
            };
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .body(template.render().unwrap())
        }
    }
}
