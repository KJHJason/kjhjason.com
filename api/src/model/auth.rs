use crate::model::base_error::Error;
use crate::model::checkbox;
use actix_web::{HttpResponse, ResponseError};
use bson::oid::ObjectId;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    pub remember: Option<checkbox::State>,
}

impl LoginData {
    pub fn remember_session(&self) -> bool {
        if self.remember.is_none() {
            return false;
        }
        self.remember.as_ref().unwrap().get_state()
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    _id: ObjectId,
    username: String,
    password: String,
}

impl User {
    pub fn new(username: String, password: String) -> User {
        User {
            _id: ObjectId::new(),
            username,
            password,
        }
    }
    pub fn get_id(&self) -> ObjectId {
        self._id.clone()
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_password(&self) -> &str {
        &self.password
    }
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub username: String,
}

#[derive(Debug, Display, Error)]
pub enum AuthError {
    #[display(fmt = "User already logged in")]
    AlreadyLoggedIn,
    #[display(fmt = "Invalid username or password")]
    UserNotFound, // same as InvalidCredentials to avoid enumeration attacks
    #[display(fmt = "Invalid username or password")]
    InvalidCredentials,
    #[display(fmt = "Internal server error")]
    InternalServerError,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let error = Error::new(self.to_string());
        match self {
            AuthError::AlreadyLoggedIn => HttpResponse::Ok().json(error),
            AuthError::UserNotFound => HttpResponse::Unauthorized().json(error),
            AuthError::InvalidCredentials => HttpResponse::Unauthorized().json(error),
            AuthError::InternalServerError => HttpResponse::InternalServerError().json(error),
        }
    }
}
