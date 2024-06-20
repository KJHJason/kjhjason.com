use crate::utils::security;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, ParamsBuilder, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use once_cell::sync::Lazy;

// follows the RFC 9106 recommendation for Argon2id
// ref: https://github.com/hynek/argon2-cffi/blob/main/src/argon2/profiles.py#L30-L38
macro_rules! get_default_hasher {
    () => {
        Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            ParamsBuilder::new()
                .t_cost(3)
                .m_cost(64 * 1024)
                .p_cost(4)
                .output_len(64)
                .build()
                .unwrap(),
        )
    };
}

// generate a random salt (cryptographically secure)
#[inline]
fn generate_salt() -> SaltString {
    let salt = security::generate_random_bytes(32);
    SaltString::encode_b64(&salt).expect("Failed to encode salt")
}

pub fn hash_password(password: &str) -> Result<String, actix_web::Error> {
    static ARGON2: Lazy<Argon2> = Lazy::new(|| get_default_hasher!());
    let salt = generate_salt();
    let output = match ARGON2.hash_password(password.as_bytes(), &salt) {
        Ok(output) => output,
        Err(err) => {
            log::error!("Failed to hash password: {:?}", err);
            return Err(actix_web::error::ErrorInternalServerError(
                "Failed to hash password",
            ));
        }
    };

    let hash_output = output.to_string();
    Ok(hash_output)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, actix_web::Error> {
    static ARGON2: Lazy<Argon2> = Lazy::new(|| get_default_hasher!());
    let password_hash = match PasswordHash::new(hash) {
        Ok(password_hash) => password_hash,
        Err(err) => {
            log::error!("Invalid password hash: {:?}", err);
            return Err(actix_web::error::ErrorInternalServerError(
                "Invalid password hash",
            ));
        }
    };

    match ARGON2.verify_password(password.as_bytes(), &password_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
