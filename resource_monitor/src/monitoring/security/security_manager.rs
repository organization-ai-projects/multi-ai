use std::sync::{Arc, Mutex};
use std::path::{PathBuf, Path};
use log::info;
use std::fs;

use crate::security::sandbox_manager::SandboxManager;
use crate::error::ResourceMonitorError;

/// Un gestionnaire de sécurité minimaliste
#[derive(Clone)]
pub struct SecurityManager {
    sandbox_manager: Arc<Mutex<SandboxManager>>,
    shared_center_path: PathBuf,
}

impl SecurityManager {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let base = base_path.as_ref().to_path_buf();
        let shared_center = base.join("shared_center");
        
        // Créer le répertoire shared_center s'il n'existe pas
        if !shared_center.exists() {
            if let Err(e) = fs::create_dir_all(&shared_center) {
                error!("Impossible de créer le shared_center: {}", e);
            }
        }
        
        Self {
            sandbox_manager: Arc::new(Mutex::new(SandboxManager::new(base))),
            shared_center_path: shared_center,
        }
    }
    
    /// Enregistre une IA et crée son sandbox
    pub fn register_ai(&self, ai_id: &str) -> Result<PathBuf, ResourceMonitorError> {
        let mut manager = self.sandbox_manager.lock().unwrap();
        
        // Créer le sandbox pour l'IA
        let sandbox_path = manager.create_sandbox(ai_id)?;
        
        // Créer aussi un dossier dans le shared_center pour cette IA
        let shared_ai_dir = self.shared_center_path.join(ai_id);
        if !shared_ai_dir.exists() {
            fs::create_dir_all(&shared_ai_dir)
                .map_err(|e| ResourceMonitorError::ConfigurationError(
                    format!("Impossible de créer le dossier de l'IA dans le shared_center: {}", e)
                ))?;
        }
        
        info!("IA {} enregistrée avec sandbox et accès au shared_center", ai_id);
        Ok(sandbox_path)
    }
    
    /// Vérifie si un chemin est dans le sandbox d'une IA
    pub fn is_path_in_sandbox(&self, ai_id: &str, path: &Path) -> bool {
        let manager = self.sandbox_manager.lock().unwrap();
        manager.is_path_in_sandbox(ai_id, path)
    }
    
    /// Obtient le chemin du sandbox d'une IA
    pub fn get_sandbox_path(&self, ai_id: &str) -> Option<PathBuf> {
        let manager = self.sandbox_manager.lock().unwrap();
        manager.get_sandbox_path(ai_id).cloned()
    }
    
    /// Obtient le chemin du shared_center
    pub fn get_shared_center_path(&self) -> &Path {
        &self.shared_center_path
    }
}
