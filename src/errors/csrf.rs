use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error as DeriveError};

#[derive(Debug, Display, DeriveError)]
pub enum CsrfError {
    #[display("CSRF token is missing")]
    MissingToken,
    #[display("CSRF token is invalid")]
    InvalidToken,
}

impl ResponseError for CsrfError {
    fn error_response(&self) -> HttpResponse {
        let error = self.to_string();
        match self {
            CsrfError::MissingToken => HttpResponse::Unauthorized().body(error),
            CsrfError::InvalidToken => HttpResponse::Unauthorized().body(error),
        }
    }
}
