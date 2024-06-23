use crate::constants::constants::{get_db_encryption_key, get_db_encryption_key_aad};
use crate::errors::crypto::CryptoError;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng, Payload},
    XChaCha20Poly1305, XNonce,
};
use once_cell::sync::Lazy;

const XNONCE_LEN: usize = 24;

pub fn encrypt(
    cipher: &XChaCha20Poly1305,
    data: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let payload = Payload { aad, msg: &data };
    let encrypted_data = match cipher.encrypt(&nonce, payload) {
        Ok(data) => data,
        Err(_) => return Err(CryptoError::EncryptionFailed),
    };

    let mut encrypted_data_with_nonce = Vec::with_capacity(encrypted_data.len() + XNONCE_LEN);
    encrypted_data_with_nonce.extend(encrypted_data);
    encrypted_data_with_nonce.extend(nonce.as_slice());
    Ok(encrypted_data_with_nonce)
}

pub fn encrypt_with_db_key(data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    static CIPHER: Lazy<XChaCha20Poly1305> = Lazy::new(|| {
        let key = get_db_encryption_key();
        XChaCha20Poly1305::new(&GenericArray::clone_from_slice(&key))
    });
    encrypt(&CIPHER, data, &get_db_encryption_key_aad())
}

pub fn decrypt(
    cipher: &XChaCha20Poly1305,
    data: &[u8],
    aad: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let data_len = data.len();
    if data_len < XNONCE_LEN || data_len == XNONCE_LEN {
        return Err(CryptoError::CiphertextTooShort);
    }

    let encrypted_data = &data[..data_len - XNONCE_LEN];
    let nonce = XNonce::from_slice(&data[data_len - XNONCE_LEN..]);
    let payload = Payload {
        aad,
        msg: encrypted_data,
    };

    match cipher.decrypt(&nonce, payload) {
        Ok(data) => Ok(data),
        Err(_) => Err(CryptoError::DecryptionFailed),
    }
}

pub fn decrypt_with_db_key(data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    static CIPHER: Lazy<XChaCha20Poly1305> = Lazy::new(|| {
        let key = get_db_encryption_key();
        XChaCha20Poly1305::new(&GenericArray::clone_from_slice(&key))
    });
    decrypt(&CIPHER, data, &get_db_encryption_key_aad())
}
