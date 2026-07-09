use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryImage {
    pub uuid: Uuid,
    pub bytes: Vec<u8>,        // Contenu binaire de l’image
    pub format: String,        // "png", "jpg", etc.
    pub timestamp: u64,
}

impl MemoryImage {
    pub fn new(bytes: Vec<u8>, format: String) -> Self {
        MemoryImage {
            uuid: Uuid::new_v7(),
            bytes,
            format,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.bytes)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryCode {
    pub uuid: Uuid,
    pub code: String,
    pub language: String,      // Langage de programmation (ex: "Rust", "Python")
    pub timestamp: u64,
}

impl MemoryCode {
    pub fn new(code: String, language: String) -> Self {
        MemoryCode {
            uuid: Uuid::new_v7(),
            code,
            language,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.code)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryLog {
    pub uuid: Uuid,
    pub content: String,       // Contenu du log
    pub timestamp: u64,
}

impl MemoryLog {
    pub fn new(content: String) -> Self {
        MemoryLog {
            uuid: Uuid::new_v7(),
            content,
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, &self.content)
    }
}
