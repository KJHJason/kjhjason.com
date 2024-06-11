use crate::constants::constants;
use crate::db;
use crate::middleware::auth;
use crate::model::auth as auth_model;
use crate::security::jwt;
use crate::security::jwt::JwtSignerLogic;
use crate::security::pw_hasher;
use crate::utils::{redirect, security};
use actix_web::cookie::{time as cookie_time, Cookie, SameSite};
use actix_web::http::header;
use actix_web::{get, post, web, web::Data, web::Json, Error, HttpRequest, HttpResponse};
use rand::Rng;
use tokio::time as tokio_time;

macro_rules! honeypot_logic {
    ($login_data:expr) => {
        log::info!(
            "Honeypot triggered! Username: {} Password: {}",
            $login_data.username,
            $login_data.password
        );
        let sleep_time = rand::thread_rng().gen_range(500..1500);
        tokio_time::sleep(tokio_time::Duration::from_millis(sleep_time)).await;
        return Err(actix_web::error::ErrorForbidden(
            "wrong username or password",
        ));
    };
}

#[post("/wp-admin.php")]
async fn wp_honeypot(login_data: Json<auth_model::LoginData>) -> Result<HttpResponse, Error> {
    honeypot_logic!(login_data);
}

#[post("/admin")]
async fn admin_honeypot(login_data: Json<auth_model::LoginData>) -> Result<HttpResponse, Error> {
    honeypot_logic!(login_data);
}

#[post("/login")]
async fn login_honeypot(login_data: Json<auth_model::LoginData>) -> Result<HttpResponse, Error> {
    honeypot_logic!(login_data);
}

#[post("auth/login")]
async fn login(
    req: HttpRequest,
    client: Data<db::DbClient>,
    login_data: web::Form<auth_model::LoginData>,
) -> Result<HttpResponse, auth_model::AuthError> {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            return Err(auth_model::AuthError::AlreadyLoggedIn);
        }
        None => {}
    }

    web::block(move || async move {
        let user = client.get_user_by_username(&login_data.username).await?;
        let is_valid = match pw_hasher::verify_password(&login_data.password, user.get_password()) {
            Ok(is_valid) => is_valid,
            Err(_) => {
                return Err(auth_model::AuthError::InternalServerError);
            }
        };
        if !is_valid {
            return Err(auth_model::AuthError::InvalidCredentials);
        }

        let signer = jwt::JwtSigner::new(
            security::get_default_jwt_key(),
            jsonwebtoken::Algorithm::HS512,
        );

        let exp_sec = if login_data.remember_session() {
            constants::SESSION_TIMEOUT_REMEMBER
        } else {
            constants::SESSION_TIMEOUT
        };
        let claims = auth::create_user_claim(user.get_id(), exp_sec);
        let token = match signer.sign(&claims) {
            Ok(token) => token,
            Err(_) => {
                return Err(auth_model::AuthError::InternalServerError);
            }
        };

        let max_age = if login_data.remember_session() {
            let offset_dt =
                cookie_time::OffsetDateTime::from_unix_timestamp(claims.exp.timestamp_micros());
            Some(offset_dt.unwrap())
        } else {
            None
        };

        let c = Cookie::build(constants::AUTH_COOKIE_NAME, token.clone())
            .domain(constants::get_domain())
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true)
            .secure(!constants::DEBUG_MODE)
            .expires(max_age)
            .finish();
        let response = auth_model::LoginResponse {
            token,
            username: user.get_username().to_string(),
        };
        return Ok(HttpResponse::Ok().cookie(c).json(response));
    })
    .await
    .unwrap()
    .await
}

#[get("/auth/logout")]
async fn logout(req: HttpRequest, redirect: web::Query<redirect::RedirectParams>) -> HttpResponse {
    let full_redirect = redirect::get_redirect_url(&redirect);
    let redirect_header = (header::LOCATION, full_redirect);
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            let c = Cookie::build(constants::AUTH_COOKIE_NAME, "")
                .domain(constants::DOMAIN)
                .path("/")
                .http_only(true)
                .secure(!constants::DEBUG_MODE)
                .finish();

            HttpResponse::SeeOther()
                .cookie(c)
                .insert_header(redirect_header)
                .finish()
        }
        None => HttpResponse::SeeOther()
            .insert_header(redirect_header)
            .finish(),
    }
}
