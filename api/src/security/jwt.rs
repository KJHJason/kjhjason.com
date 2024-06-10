use crate::constants::constants;
use derive_more::{Display, Error};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, Error)]
pub enum Errors {
    #[display(fmt = "invalid token provided")]
    InvalidToken,
    #[display(fmt = "failed to sign token")]
    FailedToSignToken,
    #[display(fmt = "token has expired")]
    TokenExpired,
}

fn get_validations() -> Validation {
    Validation::new(Algorithm::HS512)
}

fn get_secret_key() -> Vec<u8> {
    let secret_hex = std::env::var(constants::AUTH_SECRET_KEY).unwrap();
    hex::decode(secret_hex).unwrap()
}

pub trait Claim {
    fn get_exp(&self) -> chrono::DateTime<chrono::Utc>;
}

pub fn unsign<T: for<'de> Deserialize<'de> + Claim>(token: &str) -> Result<T, Errors> {
    let secret_bytes = get_secret_key();
    let token = match decode::<T>(
        &token,
        &DecodingKey::from_secret(&secret_bytes),
        &get_validations(),
    ) {
        Ok(token_data) => token_data.claims,
        Err(_) => return Err(Errors::InvalidToken),
    };

    if token.get_exp() < chrono::Utc::now() {
        return Err(Errors::TokenExpired);
    }
    Ok(token)
}

pub fn sign<T: Serialize + Claim>(claim: &T) -> Result<String, Errors> {
    let secret_bytes = get_secret_key();
    match encode(
        &jsonwebtoken::Header::default(),
        claim,
        &EncodingKey::from_secret(&secret_bytes),
    ) {
        Ok(token) => Ok(token),
        Err(err) => {
            log::error!("Failed to sign token: {:?}", err);
            Err(Errors::FailedToSignToken)
        }
    }
}
