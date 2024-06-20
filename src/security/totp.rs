use crate::constants::constants;
use crate::utils::security::generate_random_bytes;
use totp_rs::{Algorithm, Secret, TOTP};

const TOTP_DIGITS: usize = 6;
const TOTP_SKEW: u8 = 1;
const TOTP_STEP: u64 = 30; // recommended to be 30 seconds by RFC-6238

// TODO: Implement the route to set-up TOTP
#[allow(dead_code)]
pub fn generate_totp(username: &str) -> (String, String) {
    let secret_bytes = generate_random_bytes(34);
    let encoded_secret = Secret::Raw(secret_bytes.clone()).to_encoded().to_string();
    let totp = TOTP::new(
        Algorithm::SHA1,
        TOTP_DIGITS,
        TOTP_SKEW,
        TOTP_STEP,
        secret_bytes,
        Some(constants::DOMAIN.to_string()),
        username.to_string(),
    )
    .expect("Failed to create TOTP instance");
    let qr_code_data = totp.get_qr_base64().expect("failed to generate QR code");
    (encoded_secret, qr_code_data)
}

pub fn verify_totp(totp_input: &str, secret: &str) -> bool {
    let secret = Secret::Encoded(secret.to_string());
    let totp = TOTP::new(
        Algorithm::SHA1,
        TOTP_DIGITS,
        TOTP_SKEW,
        TOTP_STEP,
        secret.to_bytes().unwrap(),
        Some(constants::DOMAIN.to_string()),
        "".to_string(),
    )
    .expect("Failed to create TOTP instance");

    let token = totp
        .generate_current()
        .expect("Failed to generate TOTP token");
    totp_input == token
}
