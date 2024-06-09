use crate::constants::constants::{MAX_TAGS, MAX_THUMBNAIL_FILE_SIZE, TITLE_MAX_LENGTH};
use crate::model::base_error::Error;
use crate::utils::md;
use actix_web::{HttpResponse, ResponseError};
use bson::oid::ObjectId;
use chrono::Utc;
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BlogIdentifier {
    id: String,
}

impl BlogIdentifier {
    pub fn get_id(self) -> String {
        self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct Blog {
    _id: ObjectId,
    title: String,
    tags: Vec<String>,
    images: Vec<String>,
    content: String,
    is_public: bool,
    #[serde(with = "crate::utils::datetime::rfc3339")]
    timestamp: chrono::DateTime<Utc>,
    #[serde(with = "crate::utils::datetime::rfc3339::option")]
    last_modified: Option<chrono::DateTime<Utc>>,
    thumbnail_url: Option<String>,
}

// blog struct setter
impl Blog {
    pub fn new(
        title: String,
        content: String,
        tags: &Vec<String>,
        images: &Vec<String>,
        is_public: bool,
    ) -> Blog {
        Blog {
            _id: ObjectId::new(),
            title: title,
            tags: tags.clone(),
            images: images.clone(),
            content: content,
            is_public: is_public,
            timestamp: Utc::now(),
            last_modified: None,
            thumbnail_url: None,
        }
    }
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn get_images(&self) -> &Vec<String> {
        &self.images
    }
    pub fn get_is_public(&self) -> bool {
        self.is_public
    }
}

#[derive(Serialize)]
pub struct BlogResponse {
    id: String,
    title: String,
    content: String,
    #[serde(with = "crate::utils::datetime::rfc3339")]
    timestamp: chrono::DateTime<Utc>,
    #[serde(with = "crate::utils::datetime::rfc3339::option")]
    last_modified: Option<chrono::DateTime<Utc>>,
    thumbnail_url: Option<String>,
}

impl From<Blog> for BlogResponse {
    fn from(blog: Blog) -> Self {
        BlogResponse {
            id: blog._id.to_hex(),
            title: blog.title,
            content: md::convert_to_html(&blog.content, None),
            timestamp: blog.timestamp,
            last_modified: blog.last_modified,
            thumbnail_url: blog.thumbnail_url,
        }
    }
}

#[derive(Deserialize)]
pub struct BlogPublishOperation {
    title: String,
    tags: Vec<String>,
    images: Vec<String>,
    content: String,
    is_public: bool,
}

impl BlogPublishOperation {
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn get_images(&self) -> &Vec<String> {
        &self.images
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_is_public(&self) -> bool {
        self.is_public
    }
}

#[derive(Deserialize)]
pub struct BlogUpdateOperation {
    id: String,
    title: String,
    tags: Vec<String>,
    images: Vec<String>,
    content: String,
    is_public: Option<bool>,
}

impl BlogUpdateOperation {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }
    pub fn get_images(&self) -> &Vec<String> {
        &self.images
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }
    pub fn get_is_public(&self) -> Option<bool> {
        self.is_public
    }
}

#[derive(Serialize)]
pub struct UploadedImages {
    urls: Vec<String>,
}

impl UploadedImages {
    pub fn new(urls: Vec<String>) -> UploadedImages {
        UploadedImages { urls: urls }
    }
    pub fn append(&mut self, url: String) {
        self.urls.push(url);
    }
}

#[derive(Debug, Display)]
pub enum BlogError {
    InvalidObjectId,
    BlogNotFound,
    PublishBlogError,
    EmptyTitle,
    TitleTooLong,
    EmptyContent,
    UpdateBlogError,
    TooManyTags,
    ImageIsEmpty,
    ImageTooLarge,
    ImageUploadError,
    InternalServerError,
}

impl ResponseError for BlogError {
    fn error_response(&self) -> HttpResponse {
        match self {
            BlogError::InvalidObjectId => {
                HttpResponse::BadRequest().json(Error::new("Invalid ID".to_string()))
            }
            BlogError::BlogNotFound => {
                HttpResponse::NotFound().json(Error::new("Blog not found".to_string()))
            }
            BlogError::PublishBlogError => HttpResponse::InternalServerError()
                .json(Error::new("Failed to publish blog".to_string())),
            BlogError::EmptyTitle => {
                HttpResponse::BadRequest().json(Error::new("Title cannot be empty".to_string()))
            }
            BlogError::TitleTooLong => HttpResponse::BadRequest().json(Error::new(format!(
                "Title cannot be longer than {} characters",
                TITLE_MAX_LENGTH
            ))),
            BlogError::EmptyContent => {
                HttpResponse::BadRequest().json(Error::new("Content cannot be empty".to_string()))
            }
            BlogError::UpdateBlogError => HttpResponse::InternalServerError()
                .json(Error::new("Failed to update blog".to_string())),
            BlogError::TooManyTags => HttpResponse::BadRequest().json(Error::new(
                format!("Too many tags, must be less than {} tags", MAX_TAGS).to_string(),
            )),
            BlogError::ImageIsEmpty => {
                HttpResponse::BadRequest().json(Error::new("Image cannot be empty".to_string()))
            }
            BlogError::ImageTooLarge => HttpResponse::BadRequest().json(Error::new(
                format!(
                    "Image size must be less than {} bytes",
                    MAX_THUMBNAIL_FILE_SIZE
                )
                .to_string(),
            )),
            BlogError::ImageUploadError => HttpResponse::InternalServerError()
                .json(Error::new("Failed to upload image".to_string())),
            BlogError::InternalServerError => HttpResponse::InternalServerError()
                .json(Error::new("Internal server error".to_string())),
        }
    }
}
