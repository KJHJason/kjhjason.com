use crate::constants::constants;
use rand::Rng;

/// This assumes that the environment variables are in hex format
pub fn get_bytes_from_env(key: &str) -> Vec<u8> {
    let hex = std::env::var(key).unwrap();
    hex::decode(hex).unwrap()
}

pub fn get_default_secret_key() -> Vec<u8> {
    get_bytes_from_env(constants::SECRET_KEY)
}

pub fn get_default_salt() -> Vec<u8> {
    get_bytes_from_env(constants::SECRET_KEY_SALT)
}

pub fn get_default_key_info(salt: Vec<u8>, info: Vec<u8>) -> hmac_serialiser_rs::KeyInfo {
    hmac_serialiser_rs::KeyInfo {
        key: get_default_secret_key(),
        salt,
        info,
    }
}

pub fn get_auth_signer() -> hmac_serialiser_rs::HmacSigner {
    hmac_serialiser_rs::HmacSigner::new(
        get_default_key_info(get_default_salt(), vec![]),
        hmac_serialiser_rs::algorithm::Algorithm::SHA512,
        hmac_serialiser_rs::Encoder::UrlSafeNoPadding,
    )
}

// https://rust-random.github.io/book/guide-rngs.html
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut random_bytes = vec![0u8; length];
    rand::thread_rng().fill(&mut random_bytes[..]);
    random_bytes
}
