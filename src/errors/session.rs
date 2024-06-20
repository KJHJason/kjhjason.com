use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session not found")]
    NotFound,
    #[error("Internal server error")]
    InternalServerError,
}
