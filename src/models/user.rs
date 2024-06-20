use crate::errors::crypto::CryptoError;
use crate::security::chacha_crypto::decrypt_with_db_key;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub _id: ObjectId,
    username: String,
    password: String,
    totp_secret: Vec<u8>,
}

impl User {
    pub fn new(username: String, password: String, totp_secret: Vec<u8>) -> User {
        User {
            _id: ObjectId::new(),
            username,
            password,
            totp_secret,
        }
    }

    #[inline]
    pub fn get_password(&self) -> &str {
        &self.password
    }

    #[inline]
    pub fn has_totp(&self) -> bool {
        !self.totp_secret.is_empty()
    }

    #[inline]
    pub fn decrypt_totp_secret(&self) -> Result<Vec<u8>, CryptoError> {
        decrypt_with_db_key(&self.totp_secret)
    }
}
