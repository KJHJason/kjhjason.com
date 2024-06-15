use crate::model::checkbox;
use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use askama_actix::Template;
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
    pub fn get_password(&self) -> &str {
        &self.password
    }
}

#[derive(Template)]
#[template(path = "components/auth_success.html")]
pub(crate) struct AuthSucessTemplate<'a> {
    pub(crate) msg: &'a str,
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

#[derive(Template)]
#[template(path = "components/auth_error.html")]
struct AuthErrTemplate<'a> {
    err: &'a str,
}

impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        let content_type = ContentType::html();
        let error_html = AuthErrTemplate {
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
            AuthError::InternalServerError => HttpResponse::InternalServerError()
                .content_type(content_type)
                .body(error_html),
        }
    }
}
