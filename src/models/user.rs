use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub _id: ObjectId,
    username: String,
    email: String,
    password: String,
    totp_secret: Option<Vec<u8>>,
}

impl User {
    pub fn new(
        username: String,
        email: String,
        password: String,
        totp_secret: Option<Vec<u8>>,
    ) -> User {
        User {
            _id: ObjectId::new(),
            username,
            email,
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
        self.totp_secret.is_some()
    }

    #[inline]
    pub fn get_encrypted_totp_secret(&self) -> Option<&Vec<u8>> {
        self.totp_secret.as_ref()
    }
}
