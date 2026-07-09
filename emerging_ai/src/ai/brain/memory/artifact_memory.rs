use serde::{Serialize, Deserialize};
use uuid::Uuid;

// Gère uniquement les artefacts de mémoire.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArtifactMemory {
    pub log_uuids: Vec<Uuid>,       // UUIDs des logs
    pub image_uuids: Vec<Uuid>,     // UUIDs des images
    pub code_uuids: Vec<Uuid>,      // UUIDs des artefacts de code
    pub timestamp: u64,             // Timestamp de la dernière mise à jour
}

impl ArtifactMemory {
    pub fn new(
        log_uuids: Vec<Uuid>,
        image_uuids: Vec<Uuid>,
        code_uuids: Vec<Uuid>,
        timestamp: u64,
    ) -> Self {
        Self {
            log_uuids,
            image_uuids,
            code_uuids,
            timestamp,
        }
    }

    pub fn add_log_uuid(&mut self, log_uuid: Uuid) {
        self.log_uuids.push(log_uuid);
    }

    pub fn add_image_uuid(&mut self, image_uuid: Uuid) {
        self.image_uuids.push(image_uuid);
    }

    pub fn add_code_uuid(&mut self, code_uuid: Uuid) {
        self.code_uuids.push(code_uuid);
    }
}
