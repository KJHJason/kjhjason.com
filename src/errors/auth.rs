use crate::templates::alerts::ErrAlert;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, ResponseError};
use askama::Template;
use derive_more::{Display, Error};

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
