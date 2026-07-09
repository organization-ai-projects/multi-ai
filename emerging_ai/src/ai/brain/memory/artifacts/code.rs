use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient les artéfacts de code de l'IA.

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryCode {
    pub uuid: Uuid,
    pub code: String,          // Contenu du code
    pub language: String,      // Langage de programmation (ex: "Rust", "Python")
    pub timestamp: u64,        // Timestamp de création ou de mise à jour
}

impl MemoryCode {
    pub fn new(uuid: Uuid, code: String, language: String, timestamp: u64) -> Self {
        Self { uuid, code, language, timestamp }
    }

    pub fn update_code(&mut self, new_code: String) {
        self.code = new_code;
    }

    pub fn update_language(&mut self, new_language: String) {
        self.language = new_language;
    }
}