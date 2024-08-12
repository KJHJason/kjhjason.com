use actix_web::ResponseError;
use derive_more::{Display, Error as DeriveError};

#[derive(Debug, Display, DeriveError)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new(error: String) -> Error {
        Error { error }
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::InternalServerError().body(self.error.clone())
    }
}
