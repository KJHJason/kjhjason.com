use serde::Serialize;

#[derive(Serialize)]
pub struct GeneratedTotp {
    pub secret: String,
    pub qr_code_data: String,
}
