use crate::constants::constants;
use crate::security::jwt;
use crate::security::jwt::JwtSignerLogic;
use crate::utils::security;
use actix_web::{FromRequest, HttpRequest};
use bson::oid::ObjectId;
use jsonwebtoken::Algorithm;
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

impl UserClaim {
    fn get_signer() -> jwt::JwtSigner {
        jwt::JwtSigner::new(security::get_default_jwt_key(), Algorithm::HS512)
    }
}

pub fn create_user_claim(id: ObjectId) -> UserClaim {
    UserClaim {
        id,
        exp: chrono::Utc::now() + chrono::Duration::seconds(constants::SESSION_TIMEOUT),
    }
}

impl jwt::Claim for UserClaim {
    fn get_exp(&self) -> chrono::DateTime<chrono::Utc> {
        self.exp
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
        Box::pin(async move {
            let signer = UserClaim::get_signer();
            match signer.unsign::<UserClaim>(&auth_cookie) {
                Ok(claim) => Ok(claim),
                Err(_) => {
                    auth_failed!("Failed to unsign token");
                }
            }
        })
    }
}
