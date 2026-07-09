use crate::brain::persistent::artifacts::{log::MemoryLog, image::MemoryImage, code::MemoryCode};  // ← Depuis persistent/
use std::collections::HashMap;
use uuid::Uuid;

pub struct ArtifactCache {
    log_cache: HashMap<Uuid, MemoryLog>,
    image_cache: HashMap<Uuid, MemoryImage>,
    code_cache: HashMap<Uuid, MemoryCode>,
    dirty: bool,
}

impl ArtifactCache {
    pub fn new() -> Self {
        Self {
            log_cache: HashMap::new(),
            image_cache: HashMap::new(),
            code_cache: HashMap::new(),
            dirty: false,
        }
    }

    // Méthodes pour les logs
    pub fn get_log(&mut self, uuid: &Uuid) -> Option<&MemoryLog> {
        if !self.log_cache.contains_key(uuid) {
            if let Ok(log) = MemoryLog::load_from_bin(&format!("logs/{}.bin", uuid).into()) {
                self.log_cache.insert(*uuid, log);
            }
        }
        self.log_cache.get(uuid)
    }

    // Méthodes pour les images
    pub fn get_image(&mut self, uuid: &Uuid) -> Option<&MemoryImage> {
        if !self.image_cache.contains_key(uuid) {
            if let Ok(image) = MemoryImage::load_from_bin(&format!("images/{}.bin", uuid).into()) {
                self.image_cache.insert(*uuid, image);
            }
        }
        self.image_cache.get(uuid)
    }

    // Méthodes pour le code
    pub fn get_code(&mut self, uuid: &Uuid) -> Option<&MemoryCode> {
        if !self.code_cache.contains_key(uuid) {
            if let Ok(code) = MemoryCode::load_from_bin(&format!("codes/{}.bin", uuid).into()) {
                self.code_cache.insert(*uuid, code);
            }
        }
        self.code_cache.get(uuid)
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.dirty {
            for log in self.log_cache.values() {
                log.save_to_bin(&format!("logs/{}.bin", log.uuid).into())?;
            }
            for image in self.image_cache.values() {
                image.save_to_bin(&format!("images/{}.bin", image.uuid).into())?;
            }
            for code in self.code_cache.values() {
                code.save_to_bin(&format!("codes/{}.bin", code.uuid).into())?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.log_cache.clear();
        self.image_cache.clear();
        self.code_cache.clear();
        self.dirty = false;
    }
}
