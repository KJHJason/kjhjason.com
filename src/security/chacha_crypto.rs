use crate::constants::constants::{get_db_encryption_key, get_db_encryption_key_ad};
use crate::errors::crypto::CryptoError;
use chacha20poly1305::aead::generic_array::GenericArray;
use chacha20poly1305::{
    aead::{AeadCore, KeyInit, OsRng},
    AeadInPlace, XChaCha20Poly1305, XNonce,
};
use once_cell::sync::Lazy;

const XNONCE_LEN: usize = 24;
const TAG_LEN: usize = 16;

macro_rules! get_buffer {
    ($data:expr) => {{
        let mut buffer = Vec::with_capacity($data.len() + TAG_LEN);
        buffer.extend_from_slice($data);
        buffer
    }};
}

pub fn encrypt(cipher: &XChaCha20Poly1305, data: &[u8], ad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let nonce = XChaCha20Poly1305::generate_nonce(&mut OsRng);
    let mut buffer = get_buffer!(data);

    match cipher.encrypt_in_place(&nonce, ad, &mut buffer) {
        Ok(_) => (),
        Err(_) => return Err(CryptoError::EncryptionFailed),
    };

    let mut encrypted_data_with_nonce = Vec::with_capacity(buffer.len() + XNONCE_LEN);
    encrypted_data_with_nonce.extend_from_slice(&buffer);
    encrypted_data_with_nonce.extend_from_slice(&nonce.as_slice());
    Ok(encrypted_data_with_nonce)
}

pub fn encrypt_with_db_key(data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    static CIPHER: Lazy<XChaCha20Poly1305> = Lazy::new(|| {
        let key = get_db_encryption_key();
        XChaCha20Poly1305::new(&GenericArray::clone_from_slice(&key))
    });
    encrypt(&CIPHER, data, &get_db_encryption_key_ad())
}

pub fn decrypt(cipher: &XChaCha20Poly1305, data: &[u8], ad: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let data_len = data.len();
    if data_len < XNONCE_LEN || data_len == XNONCE_LEN {
        return Err(CryptoError::CiphertextTooShort);
    }

    let encrypted_data = &data[..data_len - XNONCE_LEN];
    let mut buffer = get_buffer!(encrypted_data);
    let nonce = XNonce::from_slice(&data[data_len - XNONCE_LEN..]);

    match cipher.decrypt_in_place(&nonce, ad, &mut buffer) {
        Ok(_) => Ok(buffer),
        Err(_) => Err(CryptoError::DecryptionFailed),
    }
}

pub fn decrypt_with_db_key(data: &[u8]) -> Result<Vec<u8>, CryptoError> {
    static CIPHER: Lazy<XChaCha20Poly1305> = Lazy::new(|| {
        // let key = Key::new_from_slice(&get_db_encryption_key())
        //     .expect("failed to parse db encryption key when trying to decrypt data");
        let key = get_db_encryption_key();
        XChaCha20Poly1305::new(&GenericArray::clone_from_slice(&key))
    });
    decrypt(&CIPHER, data, &get_db_encryption_key_ad())
}
