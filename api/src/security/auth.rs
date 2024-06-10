use crate::constants::constants;
use actix_web::{FromRequest, HttpRequest};
use bson::oid::ObjectId;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

macro_rules! auth_failed {
    ($msg:expr) => {
        log::warn!("{}", $msg);
        return Err(actix_web::error::ErrorNotFound(""));
    };
}

#[derive(Serialize, Deserialize)]
pub struct UserClaim {
    #[serde(serialize_with = "bson::serde_helpers::serialize_object_id_as_hex_string")]
    pub id: ObjectId,

    #[serde(with = "crate::utils::datetime::rfc3339")]
    pub exp: chrono::DateTime<chrono::Utc>,
}

fn get_validations() -> Validation {
    Validation::new(Algorithm::HS512)
}

fn get_secret_key() -> Vec<u8> {
    let secret_hex = std::env::var(constants::AUTH_SECRET_KEY).unwrap();
    hex::decode(secret_hex).unwrap()
}

fn get_claim(token: &str) -> Result<UserClaim, actix_web::Error> {
    let token = token.replace("Bearer ", "");
    let secret_bytes = get_secret_key();
    match decode::<UserClaim>(
        &token,
        &DecodingKey::from_secret(&secret_bytes),
        &get_validations(),
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(_) => {
            auth_failed!("Invalid token provided");
        }
    }
}

pub fn create_claim(id: ObjectId) -> UserClaim {
    UserClaim {
        id,
        exp: chrono::Utc::now() + chrono::Duration::seconds(constants::SESSION_TIMEOUT),
    }
}

pub fn sign_claim(claim: &UserClaim) -> Result<String, actix_web::Error> {
    let secret_bytes = get_secret_key();
    match jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        claim,
        &jsonwebtoken::EncodingKey::from_secret(&secret_bytes),
    ) {
        Ok(token) => Ok(token),
        Err(err) => {
            log::error!("Failed to sign token: {:?}", err);
            Err(actix_web::error::ErrorInternalServerError(
                "Failed to sign token",
            ))
        }
    }
}

impl FromRequest for UserClaim {
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_cookie = match req.cookie(constants::AUTH_COOKIE_NAME) {
            Some(cookie) => cookie.value().to_string(),
            None => {
                return Box::pin(async move {
                    auth_failed!("No auth cookie found");
                })
            }
        };

        if auth_cookie.is_empty() {
            return Box::pin(async move {
                auth_failed!("Empty auth cookie found");
            });
        }
        Box::pin(async move { get_claim(&auth_cookie) })
    }
}
