use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient les artéfacts de logs de l'IA.

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryLog {
    pub uuid: Uuid,
    pub content: String,       // Contenu du log
    pub timestamp: u64,        // Timestamp de création ou de mise à jour
}

impl MemoryLog {
    pub fn new(uuid: Uuid, content: String, timestamp: u64) -> Self {
        Self { uuid, content, timestamp }
    }

    pub fn update_content(&mut self, new_content: String) {
        self.content = new_content;
    }
}