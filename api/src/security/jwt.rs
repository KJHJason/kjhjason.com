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

pub trait Claim {
    fn get_exp(&self) -> chrono::DateTime<chrono::Utc>;
}

pub trait JwtSignerLogic {
    fn unsign<T: for<'de> Deserialize<'de> + Claim>(&self, token: &str) -> Result<T, Errors>;
    fn sign<T: Serialize + Claim>(&self, claim: &T) -> Result<String, Errors>;
}

#[derive(Clone)]
pub struct JwtSigner {
    secret_key: Vec<u8>,
    algorithm: Algorithm,

    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtSigner {
    pub fn new(secret_key: Vec<u8>, algorithm: Algorithm) -> JwtSigner {
        let encoding_key = EncodingKey::from_secret(&secret_key.clone());
        let decoding_key = DecodingKey::from_secret(&secret_key.clone());
        JwtSigner {
            secret_key,
            algorithm,
            encoding_key,
            decoding_key,
        }
    }
    fn get_validations(&self) -> Validation {
        Validation::new(self.algorithm)
    }
}

impl JwtSignerLogic for JwtSigner {
    fn unsign<T: for<'de> Deserialize<'de> + Claim>(&self, token: &str) -> Result<T, Errors> {
        let token = match decode::<T>(
            &token,
            &self.decoding_key,
            &self.get_validations(),
        ) {
            Ok(token_data) => token_data.claims,
            Err(_) => return Err(Errors::InvalidToken),
        };

        if token.get_exp() < chrono::Utc::now() {
            return Err(Errors::TokenExpired);
        }
        Ok(token)
    }

    fn sign<T: Serialize + Claim>(&self, claim: &T) -> Result<String, Errors> {
        match encode(
            &jsonwebtoken::Header::default(),
            claim,
            &self.encoding_key,
        ) {
            Ok(token) => Ok(token),
            Err(err) => {
                log::error!("Failed to sign token: {:?}", err);
                Err(Errors::FailedToSignToken)
            }
        }
    }
}
