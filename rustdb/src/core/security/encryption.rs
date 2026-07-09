use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use std::sync::Arc;

pub struct DataEncryption {
    cipher: Arc<Aes256Gcm>,
}

impl DataEncryption {
    pub fn new(key: &[u8; 32]) -> Self {
        Self {
            cipher: Arc::new(Aes256Gcm::new(key.into())),
        }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::from_slice(b"unique nonce");
        self.cipher
            .encrypt(nonce, data)
            .map_err(|e| format!("Encryption error: {}", e))
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, String> {
        let nonce = Nonce::from_slice(b"unique nonce");
        self.cipher
            .decrypt(nonce, encrypted)
            .map_err(|e| format!("Decryption error: {}", e))
    }
}
