use crate::models::file_info::FileInfo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewBlog {
    pub title: String,
    pub tags: Vec<String>,
    pub files: Vec<FileInfo>,
    pub content: String,
    pub is_public: bool,
}
