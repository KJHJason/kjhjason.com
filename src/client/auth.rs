use crate::constants;
use crate::templates::auth::Login;
use crate::utils::{html::render_template, security::extract_for_template};

use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpRequest, HttpResponse};

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
                index_page: true,
                login_url: "api/admin",
                client_login_url: "https://kjhjason.com/admin",
            };
            render_template(template, StatusCode::OK)
        }
    }
}

pub async fn login_auth(req: HttpRequest) -> HttpResponse {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => HttpResponse::TemporaryRedirect()
            .append_header((LOCATION, "/"))
            .finish(),
        None => {
            let login_uri_path = constants::get_login_uri_path();
            let template = Login {
                common: extract_for_template(&req),
                index_page: false,
                login_url: &format!("api{}", login_uri_path),
                client_login_url: &format!("https://kjhjason.com{}", login_uri_path),
            };
            render_template(template, StatusCode::OK)
        }
    }
}
