use crate::brain::memory_artifact::{MemoryImage, MemoryCode, MemoryLog};
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

pub struct ArtifactManager;

impl ArtifactManager {
    pub fn save_image(base_path: &str, image: &MemoryImage) -> String {
        let path = format!("{}/{}.{}", base_path, image.uuid, image.format);
        image.save(&path).expect("Impossible de sauvegarder l'image");
        path
    }

    pub fn save_code(base_path: &str, code: &MemoryCode) -> String {
        let path = format!("{}/{}.{}", base_path, code.uuid, "rs");
        code.save(&path).expect("Impossible de sauvegarder le code");
        path
    }

    pub fn save_log(base_path: &str, log: &MemoryLog) -> String {
        let path = format!("{}/{}.{}", base_path, log.uuid, "log");
        log.save(&path).expect("Impossible de sauvegarder le log");
        path
    }

    pub fn save_artifact(base_path: &str, content: &str, extension: &str) -> String {
        let uuid = Uuid::new_v7().to_string();
        let path = format!("{}/{}.{}", base_path, uuid, extension);
        let mut file = File::create(&path).expect("Impossible de créer le fichier artefact");
        file.write_all(content.as_bytes()).expect("Impossible d'écrire dans le fichier artefact");
        path
    }
}
