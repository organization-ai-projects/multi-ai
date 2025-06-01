use super::collections::Collection;
use bincode::{serialize, deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct StorageManager {
    pub base_path: PathBuf,
}

impl StorageManager {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    // Sauvegarde dans les deux formats
    pub fn save<T: serde::Serialize>(&self, collection: &Collection<T>) -> Result<(), String> {
        // Sauvegarde RON (pour l'humain)
        let ron_content = ron::ser::to_string_pretty(collection, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Erreur sérialisation RON : {}", e))?;
        let ron_path = self.base_path.join(format!("{}.ron", collection.name));
        fs::write(&ron_path, ron_content)
            .map_err(|e| format!("Erreur écriture RON : {}", e))?;

        // Sauvegarde binaire (pour le système)
        let bin_content = serialize(collection)
            .map_err(|e| format!("Erreur sérialisation binaire : {}", e))?;
        let bin_path = self.base_path.join(format!("{}.bin", collection.name));
        fs::write(&bin_path, bin_content)
            .map_err(|e| format!("Erreur écriture binaire : {}", e))?;

        Ok(())
    }

    // Charge selon le contexte (humain vs système)
    pub fn load<T: serde::de::DeserializeOwned>(&self, collection_name: &str, human_readable: bool) -> Result<Collection<T>, String> {
        let path = if human_readable {
            self.base_path.join(format!("{}.ron", collection_name))
        } else {
            // Vérifie si le binaire est plus récent que le RON
            let bin_path = self.base_path.join(format!("{}.bin", collection_name));
            let ron_path = self.base_path.join(format!("{}.ron", collection_name));
            
            if self.is_binary_fresh(&bin_path, &ron_path) {
                bin_path
            } else {
                // Si le binaire n'est pas à jour, on le régénère
                self.sync_binary_from_ron(collection_name)?;
                bin_path
            }
        };

        if human_readable {
            let content = fs::read_to_string(&path)
                .map_err(|e| format!("Erreur lecture : {}", e))?;
            ron::from_str(&content)
                .map_err(|e| format!("Erreur désérialisation RON : {}", e))
        } else {
            let content = fs::read(&path)
                .map_err(|e| format!("Erreur lecture : {}", e))?;
            deserialize(&content)
                .map_err(|e| format!("Erreur désérialisation binaire : {}", e))
        }
    }

    // Vérifie si le fichier binaire est à jour
    fn is_binary_fresh(&self, bin_path: &Path, ron_path: &Path) -> bool {
        let bin_modified = fs::metadata(bin_path).and_then(|m| m.modified()).ok();
        let ron_modified = fs::metadata(ron_path).and_then(|m| m.modified()).ok();
        
        match (bin_modified, ron_modified) {
            (Some(bin), Some(ron)) => bin >= ron,
            _ => false
        }
    }

    // Synchronise le binaire depuis le RON
    fn sync_binary_from_ron<T: serde::de::DeserializeOwned>(&self, collection_name: &str) -> Result<(), String> {
        let collection = self.load::<T>(collection_name, true)?;
        let bin_content = serialize(&collection)
            .map_err(|e| format!("Erreur sérialisation binaire : {}", e))?;
        let bin_path = self.base_path.join(format!("{}.bin", collection_name));
        fs::write(bin_path, bin_content)
            .map_err(|e| format!("Erreur écriture binaire : {}", e))?;
        Ok(())
    }
}
