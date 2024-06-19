use crate::constants::constants::{MAX_FILE_SIZE, MAX_TAGS, TITLE_MAX_LENGTH};
use crate::utils::{datetime, md};
use actix_web::{HttpResponse, ResponseError};
use bson::oid::ObjectId;
use bson::{doc, Bson};
use chrono::Utc;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Display, Debug)]
pub struct BlogIdentifier {
    pub id: String,
}

#[derive(Deserialize, Display, Debug)]
pub struct BlogPreview {
    content: String,
}

impl BlogPreview {
    pub fn get_content(&self) -> &str {
        &self.content
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlogProjection {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub files: Option<Vec<FileInfo>>,
    pub content: Option<String>,
    pub is_public: Option<bool>,
    pub views: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Blog {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub tags: Vec<String>,
    pub files: Vec<FileInfo>,
    pub content: String,
    pub is_public: bool,
    pub views: i64,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub timestamp: chrono::DateTime<Utc>,
    #[serde(with = "crate::utils::datetime::opt_chrono_datetime_as_bson_datetime")]
    pub last_modified: Option<chrono::DateTime<Utc>>,
}

// api struct setter
impl Blog {
    pub fn new(
        title: String,
        content: String,
        tags: &Vec<String>,
        files: &Vec<FileInfo>,
        is_public: bool,
    ) -> Blog {
        Blog {
            id: ObjectId::new(),
            title,
            tags: tags.clone(),
            files: files.clone(),
            content,
            is_public,
            views: 0,
            timestamp: Utc::now(),
            last_modified: None,
        }
    }
    pub fn get_id_string(&self) -> String {
        self.id.to_hex()
    }
    pub fn get_html_content(&self) -> String {
        md::convert_to_html(&self.content, None)
    }
    pub fn get_date_string(&self) -> String {
        // format for JavaScript to parse to the user's local timezone
        self.timestamp.to_rfc3339()
    }
    pub fn get_readable_date_diff(&self) -> String {
        datetime::get_readable_date_diff(self.timestamp)
    }
    pub fn get_last_modified_date_string(&self) -> String {
        match self.last_modified {
            Some(date) => date.to_rfc3339(),
            None => "".to_string(),
        }
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
}

impl From<Blog> for BlogResponse {
    fn from(blog: Blog) -> Self {
        BlogResponse {
            id: blog.id.to_hex(),
            title: blog.title,
            content: md::convert_to_html(&blog.content, None),
            timestamp: blog.timestamp,
            last_modified: blog.last_modified,
        }
    }
}

#[derive(Deserialize)]
pub struct BlogPublishOperation {
    pub title: String,
    pub tags: Vec<String>,
    pub files: Vec<FileInfo>,
    pub content: String,
    pub is_public: bool,
}

#[derive(Deserialize)]
pub struct BlogUpdateOperation {
    pub id: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub new_files: Option<Vec<FileInfo>>,
    pub content: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub url: String,
    pub signed_url: Option<String>,
}

impl From<FileInfo> for Bson {
    fn from(file_info: FileInfo) -> Bson {
        let doc = doc! {
            "name": file_info.name,
            "url": file_info.url,
            "signed_url": file_info.signed_url.unwrap_or_default(),
        };
        Bson::Document(doc)
    }
}

impl PartialEq for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.url == other.url
    }
}

#[derive(Serialize)]
pub struct UploadedFiles {
    files: Vec<FileInfo>,
}

impl UploadedFiles {
    pub fn new(files: Vec<FileInfo>) -> UploadedFiles {
        UploadedFiles { files }
    }
    pub fn append(&mut self, name: String, url: String, signed_url: String) {
        self.files.push(FileInfo {
            name,
            url,
            signed_url: Some(signed_url),
        });
    }
}

#[derive(Debug, Display, Error)]
pub enum BlogError {
    #[display(fmt = "Invalid ID")]
    InvalidObjectId,
    #[display(fmt = "Blog not found")]
    BlogNotFound,
    #[display(fmt = "Failed to publish blog post")]
    PublishBlogError,
    #[display(fmt = "Title cannot be empty")]
    EmptyTitle,
    #[display(fmt = "Title cannot be longer than {} characters", TITLE_MAX_LENGTH)]
    TitleTooLong,
    #[display(fmt = "Content cannot be empty")]
    EmptyContent,
    #[display(fmt = "Failed to update blog post")]
    UpdateBlogError,
    #[display(fmt = "Too many tags, must be less than {} tags", MAX_TAGS)]
    TooManyTags,
    #[display(fmt = "File cannot be empty")]
    FileIsEmpty,
    #[display(fmt = "File size must be less than {} bytes", MAX_FILE_SIZE)]
    FileTooLarge,
    #[display(fmt = "Failed to upload file")]
    FileUploadError,
    #[display(fmt = "Internal server error")]
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
