use crate::model::checkbox;
use crate::templates::alerts::ErrAlert;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use askama_actix::Template;
use bson::oid::ObjectId;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginData {
    pub username: String,
    pub password: String,
    #[serde(rename = "cf-turnstile-response")]
    pub cf_turnstile_res: String,
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
    pub fn get_password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Display, Error)]
pub enum AuthError {
    #[display(fmt = "User already logged in")]
    AlreadyLoggedIn,
    #[display(fmt = "Invalid username or password")]
    UserNotFound, // same as InvalidCredentials to avoid enumeration attacks
    #[display(fmt = "Invalid username or password")]
    InvalidCredentials,
    #[display(fmt = "Captcha verification failed")]
    CaptchaFailed,
    #[display(fmt = "Internal server error")]
    InternalServerError,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let content_type = ContentType::html();
        let error_html = ErrAlert {
            err: &self.to_string(),
        }
        .render()
        .unwrap();
        match self {
            AuthError::AlreadyLoggedIn => HttpResponse::Forbidden()
                .content_type(content_type)
                .body(error_html),
            AuthError::UserNotFound => HttpResponse::Unauthorized()
                .content_type(content_type)
                .body(error_html),
            AuthError::InvalidCredentials => HttpResponse::Unauthorized()
                .content_type(content_type)
                .body(error_html),
            AuthError::CaptchaFailed => HttpResponse::BadRequest()
                .content_type(content_type)
                .body(error_html),
            AuthError::InternalServerError => HttpResponse::InternalServerError()
                .content_type(content_type)
                .body(error_html),
        }
    }
}
