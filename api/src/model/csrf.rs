use serde::Serialize;

#[derive(Serialize)]
pub struct CsrfResponse {
    token: String,
}

impl CsrfResponse {
    pub fn new(token: String) -> CsrfResponse {
        CsrfResponse { token }
    }
}
