use crate::model::base_error::Error as BaseError;
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CsrfError {
    #[display(fmt = "CSRF token is missing")]
    MissingToken,
    #[display(fmt = "CSRF token is invalid")]
    InvalidToken,
}

impl ResponseError for CsrfError {
    fn error_response(&self) -> HttpResponse {
        let error = BaseError::new(self.to_string());
        match self {
            CsrfError::MissingToken => HttpResponse::Unauthorized().json(error),
            CsrfError::InvalidToken => HttpResponse::Unauthorized().json(error),
        }
    }
}
