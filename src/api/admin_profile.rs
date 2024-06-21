use crate::database::db;
use crate::errors::auth::AuthError;
use crate::middleware::auth::get_user_claim;
use crate::models::setup_2fa::Setup2fa;
use crate::security::chacha_crypto;
use crate::security::totp;
use crate::templates::admin_profile::Disable2FA;
use crate::utils::html::render_template;
use crate::utils::security::get_csrf_header_json;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Form};
use actix_web::{get, post, HttpRequest, HttpResponse};
use bson::doc;
use mongodb::options::FindOneOptions;

#[get("/api/admin/generate-2fa")]
async fn generate_2fa(
    client: Data<db::DbClient>,
    req: HttpRequest,
) -> Result<HttpResponse, AuthError> {
    let user_info = get_user_claim(&req);

    let options = FindOneOptions::builder()
        .projection(doc! {"email": 1})
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
    let user_info = get_user_claim(&req);

    let secret = &setup_data.secret;
    let totp_code = &setup_data.totp_code;

    let valid_totp = totp::verify_totp(totp_code, secret);
    if !valid_totp {
        return Err(AuthError::InvalidTotp);
    }

    let encrypted_secret = chacha_crypto::encrypt_with_db_key(secret.as_bytes()).map_err(|e| {
        log::error!("Failed to encrypt totp secret: {:?}", e);
        AuthError::InternalServerError
    })?;
    let encrypted_secret = bson::Binary {
        subtype: bson::spec::BinarySubtype::Generic,
        bytes: encrypted_secret,
    };

    client
        .get_user_collection()
        .update_one(
            doc! {"_id": user_info.user_id},
            doc! {"$set": {"totp_secret": encrypted_secret}},
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
