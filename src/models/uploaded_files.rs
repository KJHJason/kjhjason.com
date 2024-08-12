use crate::models::file_info::FileInfo;

use serde::Serialize;

#[derive(Serialize)]
pub struct UploadedFiles {
    files: Vec<FileInfo>,
}

impl UploadedFiles {
    pub fn new(files: Vec<FileInfo>) -> UploadedFiles {
        UploadedFiles { files }
    }

    #[inline]
    pub fn append(&mut self, name: String, url: String, signed_url: String) {
        self.files.push(FileInfo {
            name,
            url,
            signed_url: Some(signed_url),
        });
    }
}
