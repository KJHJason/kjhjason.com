use argon2::{
    password_hash::SaltString, Algorithm, Argon2, ParamsBuilder, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use rand::{thread_rng, Rng};

// follows the RFC 9106 recommendation for Argon2id
// ref: https://github.com/hynek/argon2-cffi/blob/main/src/argon2/profiles.py#L30-L38
fn get_default_hasher() -> Argon2<'static> {
    let params = ParamsBuilder::new()
        .t_cost(3)
        .m_cost(64 * 1024)
        .p_cost(4)
        .output_len(64)
        .build()
        .unwrap();
    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

// generate a random salt (cryptographically secure)
fn generate_salt() -> SaltString {
    let mut salt = [0u8; 32];
    thread_rng().fill(&mut salt[..]);
    SaltString::encode_b64(&salt).expect("Failed to encode salt")
}

pub fn hash_password(password: &str) -> Result<String, actix_web::Error> {
    let argon2 = get_default_hasher();
    let salt = generate_salt();
    let output = match argon2.hash_password(password.as_bytes(), &salt) {
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
    let argon2 = get_default_hasher();
    let password_hash = match PasswordHash::new(hash) {
        Ok(password_hash) => password_hash,
        Err(err) => {
            log::error!("Invalid password hash: {:?}", err);
            return Err(actix_web::error::ErrorInternalServerError(
                "Invalid password hash",
            ));
        }
    };

    match argon2.verify_password(password.as_bytes(), &password_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false),
    }
}
