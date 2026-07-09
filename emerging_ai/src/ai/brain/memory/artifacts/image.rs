use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient les artéfacts d'image de l'IA.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryImage {
    pub uuid: Uuid,
    pub bytes: Vec<u8>,        // Contenu binaire de l’image
    pub format: String,        // "png", "jpg", etc.
    pub timestamp: u64,        // Timestamp de création ou de mise à jour
}

impl MemoryImage {
    pub fn new(uuid: Uuid, bytes: Vec<u8>, format: String, timestamp: u64) -> Self {
        Self { uuid, bytes, format, timestamp }
    }

    pub fn update_bytes(&mut self, new_bytes: Vec<u8>) {
        self.bytes = new_bytes;
    }

    pub fn update_format(&mut self, new_format: String) {
        self.format = new_format;
    }
}