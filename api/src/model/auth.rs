use crate::model::base_error::Error;
use actix_web::{HttpResponse, ResponseError};
use bson::oid::ObjectId;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
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
    pub token: String,
    pub username: String,
}

#[derive(Debug, Display)]
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
            AuthError::UserNotFound => HttpResponse::Forbidden().json(error),
            AuthError::InvalidCredentials => HttpResponse::Forbidden().json(error),
            AuthError::InternalServerError => HttpResponse::InternalServerError().json(error),
        }
    }
}
