use serde::Serialize;

#[derive(Serialize)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new(error: String) -> Error {
        Error { error: error }
    }
}
