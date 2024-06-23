use crate::database::db;
use crate::errors::auth::AuthError;
use crate::middleware::auth::get_user_claim;
use crate::models::change_password::ChangePassword;
use crate::models::remove_2fa::Remove2fa;
use crate::models::setup_2fa::Setup2fa;
use crate::models::user;
use crate::security::cf_turnstile;
use crate::security::totp;
use crate::security::{chacha_crypto, pw_hasher};
use crate::templates::admin_profile::{Disable2FA, Enable2FA};
use crate::templates::alerts::SuccessAlert;
use crate::utils::auth::cf_turnstile::verify_captcha;
use crate::utils::html::render_template;
use crate::utils::security::get_csrf_header_json;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Form};
use actix_web::{get, patch, post, web, HttpRequest, HttpResponse};
use bson::doc;
use mongodb::options::FindOneOptions;

#[get("/api/admin/generate-2fa")]
async fn generate_2fa(
    client: Data<db::DbClient>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    let user_info = get_user_claim(&req);

    let options = FindOneOptions::builder()
        .projection(doc! {user::TOTP_SECRET_KEY: 1, user::EMAIL_KEY: 1})
        .build();
    let user = client
        .get_projected_user_by_id(&user_info.user_id, Some(options))
        .await?;

    let generated_totp = totp::generate_totp(&user.email.unwrap_or_default());
    Ok(HttpResponse::Ok().json(generated_totp))
}

#[post("/api/admin/setup-2fa")]
async fn setup_2fa(
    client: Data<db::DbClient>,
    req: HttpRequest,
    setup_data: Form<Setup2fa>,
) -> Result<HttpResponse, AuthError> {
    verify_captcha!(&req, &setup_data.cf_turnstile_res);

    let user_info = get_user_claim(&req);
    let secret = &setup_data.secret;
    let totp_code = &setup_data.totp_code;
    let valid_totp = totp::verify_totp(totp_code, secret);
    if !valid_totp {
        return Err(AuthError::InvalidTotp);
    }

    let options = FindOneOptions::builder()
        .projection(doc! {user::TOTP_SECRET_KEY: 1, user::PASSWORD_KEY: 1})
        .build();
    let user_doc = client
        .get_projected_user_by_id(&user_info.user_id, Some(options))
        .await?;
    if !user_doc.totp_secret.unwrap_or_default().is_empty() {
        return Err(AuthError::AlreadyEnabled2fa);
    }

    // initialise for the blocking operation to consume
    let user_password_hash = user_doc.password.unwrap_or_default();
    let login_data_password = setup_data.current_password.to_string();
    let secret_bytes = secret.as_bytes().to_vec();
    let encrypted_secret = web::block(move || {
        pw_hasher::verify_user_password(&login_data_password, &user_password_hash, false)?;

        let encrypted_secret = match chacha_crypto::encrypt_with_db_key(&secret_bytes) {
            Ok(encrypted_secret) => encrypted_secret,
            Err(e) => {
                log::error!("Failed to encrypt totp secret: {:?}", e);
                return Err(AuthError::InternalServerError);
            }
        };
        let encrypted_secret = bson::Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: encrypted_secret,
        };
        Ok(encrypted_secret)
    })
    .await
    .map_err(|e| {
        log::error!(
            "Blocking Error when trying to verify user's password and to encrypt totp secret: {:?}",
            e
        );
        AuthError::InternalServerError
    })??;

    client
        .get_user_collection()
        .update_one(
            doc! {"_id": user_info.user_id},
            doc! {"$set": {user::TOTP_SECRET_KEY: encrypted_secret}},
            None,
        )
        .await
        .map_err(|e| {
            log::error!("Failed to set user's totp secret: {:?}", e);
            AuthError::InternalServerError
        })?;

    let template = Disable2FA {
        csrf_header_json: get_csrf_header_json(&req, None),
    };
    Ok(render_template(template, StatusCode::OK))
}

#[post("/api/admin/remove-2fa")]
async fn remove_2fa(
    client: Data<db::DbClient>,
    req: HttpRequest,
    remove_data: Form<Remove2fa>,
) -> Result<HttpResponse, AuthError> {
    verify_captcha!(&req, &remove_data.cf_turnstile_res);

    let user_info = get_user_claim(&req);
    let options = FindOneOptions::builder()
        .projection(doc! {user::TOTP_SECRET_KEY: 1, user::PASSWORD_KEY: 1})
        .build();
    let user_doc = client
        .get_projected_user_by_id(&user_info.user_id, Some(options))
        .await?;
    if user_doc.totp_secret.unwrap_or_default().is_empty() {
        return Err(AuthError::AlreadyDisabled2fa);
    }

    let password_hash = user_doc.password.unwrap_or_default();
    let current_password = remove_data.current_password.to_string();
    let _ = web::block(move || {
        pw_hasher::verify_user_password(&current_password, &password_hash, false)
    })
    .await
    .map_err(|e| {
        log::error!(
            "Blocking Error when trying to verify user's password: {:?}",
            e
        );
        AuthError::InternalServerError
    })??;

    client
        .get_user_collection()
        .update_one(
            doc! {"_id": user_info.user_id},
            doc! {"$set": {user::TOTP_SECRET_KEY: bson::Bson::Null}},
            None,
        )
        .await
        .map_err(|e| {
            log::error!("Failed to remove user's totp secret: {:?}", e);
            AuthError::InternalServerError
        })?;

    let template = Enable2FA {
        csrf_header_json: get_csrf_header_json(&req, None),
    };
    Ok(render_template(template, StatusCode::OK))
}

#[patch("/api/admin/change-password")]
async fn change_password(
    client: Data<db::DbClient>,
    req: HttpRequest,
    change_data: Form<ChangePassword>,
) -> Result<HttpResponse, AuthError> {
    verify_captcha!(&req, &change_data.cf_turnstile_res);

    if change_data.new_password != change_data.confirm_password {
        return Err(AuthError::PasswordMismatch);
    }

    let user_info = get_user_claim(&req);
    let options = FindOneOptions::builder()
        .projection(doc! {user::TOTP_SECRET_KEY: 1, user::PASSWORD_KEY: 1})
        .build();
    let user_doc = client
        .get_projected_user_by_id(&user_info.user_id, Some(options))
        .await?;

    let password_hash = user_doc.password.unwrap_or_default();
    let current_password = change_data.current_password.to_string();
    let new_password = change_data.new_password.to_string();
    let new_password_hash = web::block(move || {
        pw_hasher::verify_user_password(&current_password, &password_hash, false)?;

        let new_password_hash = match pw_hasher::hash_password(&new_password) {
            Ok(hash) => hash,
            Err(_) => {
                return Err(AuthError::InternalServerError);
            }
        };
        Ok(new_password_hash)
    })
    .await
    .map_err(|e| {
        log::error!(
            "Blocking Error when trying to verify user's password and hashing user's new password: {:?}",
            e
        );
        AuthError::InternalServerError
    })??;

    client
        .get_user_collection()
        .update_one(
            doc! {"_id": user_info.user_id},
            doc! {"$set": {user::PASSWORD_KEY: new_password_hash}},
            None,
        )
        .await
        .map_err(|e| {
            log::error!("Failed to update user's password: {:?}", e);
            AuthError::InternalServerError
        })?;

    let template = SuccessAlert {
        msg: "Password changed successfully",
    };
    Ok(render_template(template, StatusCode::OK))
}
