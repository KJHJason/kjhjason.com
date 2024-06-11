use rand::Rng;
use crate::constants::constants;

pub fn get_default_jwt_key() -> Vec<u8> {
    let secret_hex = std::env::var(constants::JWT_SECRET_KEY).unwrap();
    hex::decode(secret_hex).unwrap()
}

pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    let mut random_bytes = vec![0u8; length];
    rand::thread_rng().fill(&mut random_bytes[..]);
    random_bytes
}
