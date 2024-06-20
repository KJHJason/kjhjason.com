use crate::model::file_info::FileInfo;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectedBlog {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub title: Option<String>,
    pub tags: Option<Vec<String>>,
    pub files: Option<Vec<FileInfo>>,
    pub content: Option<String>,
    pub is_public: Option<bool>,
    pub views: Option<i64>,
}
