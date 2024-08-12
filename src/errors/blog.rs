use crate::constants::{MAX_FILE_SIZE, MAX_TAGS, TITLE_MAX_LENGTH};

use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error as DeriveError};

#[derive(Debug, Display, DeriveError)]
pub enum BlogError {
    #[display("Invalid ID")]
    InvalidObjectId,
    #[display("Blog not found")]
    BlogNotFound,
    #[display("Failed to publish blog post")]
    PublishBlogError,
    #[display("Title cannot be empty")]
    EmptyTitle,
    #[display("Title cannot be longer than {} characters", TITLE_MAX_LENGTH)]
    TitleTooLong,
    #[display("Content cannot be empty")]
    EmptyContent,
    #[display("Failed to update blog post")]
    UpdateBlogError,
    #[display("Too many tags, must be less than {} tags", MAX_TAGS)]
    TooManyTags,
    #[display("File cannot be empty")]
    FileIsEmpty,
    #[display("File size must be less than {} bytes", MAX_FILE_SIZE)]
    FileTooLarge,
    #[display("Failed to upload file")]
    FileUploadError,
    #[display("Internal server error")]
    InternalServerError,
}

impl ResponseError for BlogError {
    fn error_response(&self) -> HttpResponse {
        let error = self.to_string();
        match self {
            BlogError::InvalidObjectId => HttpResponse::BadRequest().body(error),
            BlogError::BlogNotFound => HttpResponse::NotFound().body(error),
            BlogError::PublishBlogError => HttpResponse::InternalServerError().body(error),
            BlogError::EmptyTitle => HttpResponse::BadRequest().body(error),
            BlogError::TitleTooLong => HttpResponse::BadRequest().body(error),
            BlogError::EmptyContent => HttpResponse::BadRequest().body(error),
            BlogError::UpdateBlogError => HttpResponse::InternalServerError().body(error),
            BlogError::TooManyTags => HttpResponse::BadRequest().body(error),
            BlogError::FileIsEmpty => HttpResponse::BadRequest().body(error),
            BlogError::FileTooLarge => HttpResponse::BadRequest().body(error),
            BlogError::FileUploadError => HttpResponse::InternalServerError().body(error),
            BlogError::InternalServerError => HttpResponse::InternalServerError().body(error),
        }
    }
}
