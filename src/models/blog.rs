use crate::models::file_info::FileInfo;
use crate::utils::{datetime, md};
use bson::oid::ObjectId;
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub const TITLE_KEY: &str = "title";
pub const SEO_DESC_KEY: &str = "seo_desc";
pub const TAGS_KEY: &str = "tags";
pub const FILES_KEY: &str = "files";
pub const CONTENT_KEY: &str = "content";
pub const IS_PUBLIC_KEY: &str = "is_public";
pub const VIEWS_KEY: &str = "views";
pub const LAST_MODIFIED_KEY: &str = "last_modified";

#[derive(Serialize, Deserialize, Clone)]
pub struct Blog {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub title: String,
    pub seo_desc: String,
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
        seo_desc: String,
        content: String,
        tags: &Vec<String>,
        files: &Vec<FileInfo>,
        is_public: bool,
    ) -> Blog {
        Blog {
            id: ObjectId::new(),
            title,
            seo_desc: seo_desc,
            tags: tags.clone(),
            files: files.clone(),
            content,
            is_public,
            views: 0,
            timestamp: Utc::now(),
            last_modified: None,
        }
    }

    #[inline]
    pub fn get_id_string(&self) -> String {
        self.id.to_hex()
    }

    #[inline]
    pub fn get_html_content(&self) -> String {
        md::convert_to_html(&self.content, None)
    }

    #[inline]
    pub fn get_date_string(&self) -> String {
        // format for JavaScript to parse to the user's local timezone
        self.timestamp.to_rfc3339()
    }

    #[inline]
    pub fn get_readable_date_diff(&self) -> String {
        datetime::get_readable_date_diff(self.timestamp)
    }

    #[inline]
    pub fn get_last_modified_date_string(&self) -> String {
        match self.last_modified {
            Some(date) => date.to_rfc3339(),
            None => "".to_string(),
        }
    }
}
