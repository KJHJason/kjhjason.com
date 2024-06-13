use crate::constants::constants;
use crate::db;
use crate::middleware::auth;
use crate::model::auth as auth_model;
use crate::security::pw_hasher;
use crate::utils::security;
use actix_web::cookie::{time as cookie_time, Cookie, SameSite};
use actix_web::{post, web, web::Data, web::Form, HttpRequest, HttpResponse};
use hmac_serialiser_rs::SignerLogic;
use rand::Rng;
use tokio::time as tokio_time;

#[post("/admin")]
async fn admin_honeypot(
    login_data: Form<auth_model::LoginData>,
) -> Result<HttpResponse, auth_model::AuthError> {
    log::warn!(
        "Honeypot triggered! Username: {} Password: {}",
        login_data.username,
        login_data.password
    );
    let sleep_time = rand::thread_rng().gen_range(500..1500);
    tokio_time::sleep(tokio_time::Duration::from_millis(sleep_time)).await;
    Err(auth_model::AuthError::InvalidCredentials)
}

#[post("auth/login")]
async fn login(
    req: HttpRequest,
    client: Data<db::DbClient>,
    login_data: Form<auth_model::LoginData>,
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

        let signer = security::get_auth_signer();
        let exp_sec = if login_data.remember_session() {
            constants::SESSION_TIMEOUT_REMEMBER
        } else {
            constants::SESSION_TIMEOUT
        };
        let claims = auth::create_user_claim(user.get_id(), exp_sec);
        let token = signer.sign(&claims);

        let max_age = if login_data.remember_session() {
            let offset_dt =
                cookie_time::OffsetDateTime::from_unix_timestamp(claims.exp.timestamp_millis());
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
            username: user.get_username().to_string(),
        };
        return Ok(HttpResponse::Ok().cookie(c).json(response));
    })
    .await
    .unwrap()
    .await
}

#[post("/auth/logout")]
async fn logout(req: HttpRequest) -> HttpResponse {
    let msg = "you have logged out";
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            let mut c = Cookie::build(constants::AUTH_COOKIE_NAME, "")
                .domain(constants::get_domain())
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .secure(!constants::DEBUG_MODE)
                .finish();
            c.make_removal();
            HttpResponse::Ok().cookie(c).body(msg)
        }
        None => HttpResponse::Ok().body(msg),
    }
}
