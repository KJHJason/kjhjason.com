use crate::constants::constants;
use crate::database::db;
use crate::errors::auth::AuthError;
use crate::middleware::auth;
use crate::models::{login_data::LoginData, session::Session};
use crate::security::cf_turnstile;
use crate::security::chacha_crypto::decrypt_with_db_key;
use crate::security::pw_hasher;
use crate::security::totp;
use crate::templates;
use crate::utils::auth::cf_turnstile::verify_captcha;
use crate::utils::html::render_template;
use actix_web::cookie::{time as cookie_time, Cookie, SameSite};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{post, web, web::Data, web::Form, HttpRequest, HttpResponse};
use askama::Template;
use rand::Rng;
use tokio::time as tokio_time;

#[post("/api/admin")]
async fn admin_honeypot(
    req: HttpRequest,
    login_data: Form<LoginData>,
) -> Result<HttpResponse, AuthError> {
    log::warn!(
        "Honeypot triggered! Request IP: {} Username: {} Password: {}",
        cf_turnstile::get_ip_addr(&req).unwrap_or("unknown".to_string()),
        login_data.username,
        login_data.password
    );
    verify_captcha!(&req, &login_data.cf_turnstile_res);
    let sleep_time = rand::thread_rng().gen_range(2000..4000);
    tokio_time::sleep(tokio_time::Duration::from_millis(sleep_time)).await;
    Err(AuthError::InvalidCredentials)
}

#[post("/api/auth/login")]
async fn login(
    req: HttpRequest,
    client: Data<db::DbClient>,
    login_data: Form<LoginData>,
) -> Result<HttpResponse, AuthError> {
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            return Err(AuthError::AlreadyLoggedIn);
        }
        None => {}
    }
    verify_captcha!(&req, &login_data.cf_turnstile_res);

    let login_data_password = &login_data.password;
    if login_data_password.len() > 64 {
        return Err(AuthError::InvalidCredentials);
    }
    let login_data_totp_input = match &login_data.totp_input {
        Some(totp_input) => totp_input.to_string(),
        None => "".to_string(),
    };
    if login_data_totp_input.len() > 6 {
        return Err(AuthError::InvalidCredentials);
    }

    let user = client
        .get_user_by_username_or_email(&login_data.username)
        .await?;

    // Initialise the variables to be consumed by the blocking operation
    let login_data_password = login_data_password.to_string();
    let user_password_hash = user.get_password().to_string();
    let user_has_totp = user.has_totp();
    let user_totp_secret = if user_has_totp {
        match user.get_encrypted_totp_secret() {
            Some(user_totp_secret) => user_totp_secret.to_vec(),
            None => {
                log::error!("User has TOTP enabled but no TOTP secret found");
                return Err(AuthError::InternalServerError);
            }
        }
    } else {
        vec![]
    };

    web::block(move || {
        pw_hasher::verify_user_password(&login_data_password, &user_password_hash, true)?;
        if user_has_totp {
            let decrypted_totp = match decrypt_with_db_key(&user_totp_secret) {
                Ok(decrypted_totp) => {
                    String::from_utf8(decrypted_totp).expect("totp secret should be valid utf-8")
                }
                Err(e) => {
                    log::error!("Failed to decrypt TOTP: {:?}", e);
                    return Err(AuthError::InternalServerError);
                }
            };

            if login_data_totp_input.is_empty() {
                return Err(AuthError::MissingTotp);
            }
            if !totp::verify_totp(&login_data_totp_input, &decrypted_totp) {
                return Err(AuthError::InvalidTotp);
            }
        }
        Ok(())
    })
    .await
    .map_err(|e| {
        log::error!("Blocking Error when verifying password and TOTP: {:?}", e);
        AuthError::InternalServerError
    })??;

    let exp_sec = if login_data.remember_session() {
        constants::SESSION_TIMEOUT_REMEMBER
    } else {
        constants::SESSION_TIMEOUT
    };
    let session_col = client.get_session_collection();
    let session = Session::new(user._id, exp_sec);
    let session_expiry = session.expiry.timestamp_millis();
    let result = match session_col.insert_one(session, None).await {
        Ok(result) => result,
        Err(e) => {
            log::error!("Failed to insert session into db: {:?}", e);
            return Err(AuthError::InternalServerError);
        }
    };

    let claims = auth::create_user_claim(user._id, result.inserted_id.as_object_id().unwrap());
    let token = auth::sign_payload(&claims);
    let max_age = if login_data.remember_session() {
        // offset_dt is 10 seconds before the EXPIRY time for extra leeway
        let offset_dt = cookie_time::OffsetDateTime::from_unix_timestamp(session_expiry - 10_000);
        Some(offset_dt.unwrap())
    } else {
        None
    };

    let c = Cookie::build(constants::AUTH_COOKIE_NAME, token.clone())
        .domain(constants::get_domain())
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .secure(!constants::get_debug_mode())
        .expires(max_age)
        .finish();
    let template = templates::alerts::SuccessAlert {
        msg: "You have logged in",
    };
    let mut response = render_template(template, StatusCode::OK);
    response.add_cookie(&c).unwrap();
    response
        .headers_mut()
        .insert("HX-Redirect".parse().unwrap(), "/".parse().unwrap());
    return Ok(response);
}

#[post("/api/logout")]
async fn logout(req: HttpRequest) -> HttpResponse {
    let html = templates::guest::GuestItems.render().unwrap();
    match req.cookie(constants::AUTH_COOKIE_NAME) {
        Some(_) => {
            let mut auth_cookie = Cookie::build(constants::AUTH_COOKIE_NAME, "")
                .domain(constants::get_domain())
                .path("/")
                .http_only(true)
                .same_site(SameSite::Lax)
                .secure(!constants::get_debug_mode())
                .finish();
            auth_cookie.make_removal();
            HttpResponse::Ok()
                .content_type(ContentType::html())
                .cookie(auth_cookie)
                .body(html)
        }
        None => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(html),
    }
}
