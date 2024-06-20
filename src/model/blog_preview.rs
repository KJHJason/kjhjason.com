use derive_more::Display;
use serde::Deserialize;

#[derive(Deserialize, Display, Debug)]
pub struct BlogPreview {
    content: String,
}

impl BlogPreview {
    pub fn get_content(&self) -> &str {
        &self.content
    }
}
