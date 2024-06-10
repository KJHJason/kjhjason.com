use crate::db;
use crate::model::auth as auth_model;
use crate::security::auth;
use crate::security::pw_hasher;
use actix_web::cookie::time;
use actix_web::{
    cookie::Cookie, post, web, web::Data, web::Json, Error, HttpRequest, HttpResponse,
};
use rand::Rng;
// use time::Duration;
use crate::constants::constants;
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
    login_data: Json<auth_model::LoginData>,
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

        let claims = auth::create_claim(user.get_id());
        let token = match auth::sign_claim(&claims) {
            Ok(token) => token,
            Err(_) => {
                return Err(auth_model::AuthError::InternalServerError);
            }
        };

        let max_age = claims.exp.timestamp() - chrono::Utc::now().timestamp();
        let domain = if constants::DEBUG_MODE {
            "localhost"
        } else {
            constants::DOMAIN
        };
        let c = Cookie::build(constants::AUTH_COOKIE_NAME, token.clone())
            .domain(domain)
            .path("/")
            .http_only(true)
            .secure(!constants::DEBUG_MODE)
            .max_age(time::Duration::seconds(max_age))
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
