use serde::Serialize;

#[derive(Serialize)]
pub struct Index {
    name: String,
    version: String,
    frontend_url: String,
}

impl Index {
    pub fn new() -> Index {
        Index {
            name: "KJHJason's Blog Backend API".to_string(),
            version: "0.1.0".to_string(),
            frontend_url: "https://kjhjason.com".to_string(),
        }
    }
}
