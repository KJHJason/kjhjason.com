use crate::models::file_info::FileInfo;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateBlog {
    pub id: String,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub new_files: Option<Vec<FileInfo>>,
    pub content: Option<String>,
    pub is_public: Option<bool>,
}
