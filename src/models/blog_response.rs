use crate::models::blog::Blog;
use crate::utils::md;
use chrono::Utc;
use serde::Serialize;

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
