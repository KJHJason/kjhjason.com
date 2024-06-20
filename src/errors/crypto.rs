use thiserror::Error;

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("failed to encrypt plaintext")]
    EncryptionFailed,
    #[error("ciphertext too short")]
    CiphertextTooShort,
    #[error("failed to decrypt ciphertext")]
    DecryptionFailed,
}
