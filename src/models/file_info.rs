use bson::{doc, Bson};
use serde::{Deserialize, Serialize};

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
