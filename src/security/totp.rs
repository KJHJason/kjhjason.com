use crate::constants::constants;
use crate::models::generated_totp::GeneratedTotp;
use crate::utils::security::generate_random_bytes;
use totp_rs::{Algorithm, Secret, TOTP};

const TOTP_SECRET_LEN: usize = 34; // bytes (16 bytes minimum but >=20 bytes recommended by RFC-4226)
const TOTP_DIGITS: usize = 6;
const TOTP_SKEW: u8 = 1;
const TOTP_STEP: u64 = 30; // recommended to be 30 seconds by RFC-6238

pub fn generate_totp(username: &str) -> GeneratedTotp {
    let secret_bytes = generate_random_bytes(TOTP_SECRET_LEN);
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
    .expect("TOTP instance should be created without errors");

    let qr_code_data = totp
        .get_qr_base64()
        .expect("Should be able to generate QR code");
    GeneratedTotp {
        secret: encoded_secret,
        qr_code_data,
    }
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
    .expect("TOTP instance should be created without errors");

    let token = totp
        .generate_current()
        .expect("Should be able to generate TOTP token for verification");
    totp_input == token
}
