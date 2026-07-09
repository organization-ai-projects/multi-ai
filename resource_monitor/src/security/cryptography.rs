use ring::{aead, digest, pbkdf2, rand};
use ring::rand::SecureRandom;
use std::num::NonZeroU32;
use log::error;

pub struct CryptographyProvider {
    key: aead::LessSafeKey,
    nonce_generator: rand::SystemRandom,
}

impl CryptographyProvider {
    pub fn new(passphrase: &[u8]) -> Result<Self, String> {
        // Générer une clé à partir du mot de passe
        let salt = b"resource_monitor_salt"; // Idéalement devrait être aléatoire et stocké
        let iterations = NonZeroU32::new(100_000).unwrap();
        let mut key_bytes = [0u8; 32]; // 256 bits
        
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256, 
            iterations, 
            salt, 
            passphrase, 
            &mut key_bytes
        );
        
        // Créer la clé
        let unbound_key = match aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &key_bytes) {
            Ok(k) => k,
            Err(_) => return Err("Erreur lors de la création de la clé".to_string()),
        };
        
        let key = aead::LessSafeKey::new(unbound_key);
        let nonce_generator = rand::SystemRandom::new();
        
        Ok(Self { key, nonce_generator })
    }
    
    /// Chiffre un message avec la clé
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // Générer un nonce (nombre utilisé une seule fois)
        let mut nonce_bytes = [0u8; 12];
        if let Err(_) = self.nonce_generator.fill(&mut nonce_bytes) {
            return Err("Erreur lors de la génération du nonce".to_string());
        }
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        
        // Préparer le message chiffré avec le nonce en préfixe
        let mut encrypted = nonce_bytes.to_vec();
        let mut buffer = data.to_vec();
        
        // Chiffrer en place
        match self.key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut buffer) {
            Ok(_) => {
                encrypted.append(&mut buffer);
                Ok(encrypted)
            },
            Err(_) => Err("Erreur lors du chiffrement".to_string()),
        }
    }
    
    /// Déchiffre un message
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, String> {
        if encrypted_data.len() < 12 {
            return Err("Données chiffrées trop courtes".to_string());
        }
        
        // Extraire le nonce (12 premiers octets)
        let nonce_bytes: [u8; 12] = encrypted_data[0..12].try_into().unwrap();
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        
        // Le reste est le message chiffré
        let mut ciphertext = encrypted_data[12..].to_vec();
        
        // Déchiffrer en place
        match self.key.open_in_place(nonce, aead::Aad::empty(), &mut ciphertext) {
            Ok(plaintext) => {
                Ok(plaintext.to_vec())
            },
            Err(_) => Err("Erreur lors du déchiffrement".to_string()),
        }
    }
    
    /// Génère un hash SHA-256 d'un message
    pub fn hash(&self, data: &[u8]) -> Vec<u8> {
        let hash = digest::digest(&digest::SHA256, data);
        hash.as_ref().to_vec()
    }
    
    /// Vérifie l'intégrité d'un message avec son hash
    pub fn verify_hash(&self, data: &[u8], expected_hash: &[u8]) -> bool {
        let hash = self.hash(data);
        hash == expected_hash
    }
}
