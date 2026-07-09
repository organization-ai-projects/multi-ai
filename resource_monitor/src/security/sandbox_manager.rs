use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use log::{info, warn};

use crate::error::ResourceMonitorError;

/// Gestionnaire des chemins sandbox pour les IA - minimal
/// Ne s'occupe QUE des chemins, pas du contenu ni de l'organisation
pub struct SandboxManager {
    base_path: PathBuf,
    sandbox_paths: HashMap<String, PathBuf>,
}

impl SandboxManager {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let base = base_path.as_ref().to_path_buf();
        
        Self {
            base_path: base,
            sandbox_paths: HashMap::new(),
        }
    }
    
    /// Crée un répertoire sandbox pour une IA - sans structure imposée
    pub fn create_sandbox(&mut self, ai_id: &str) -> Result<PathBuf, ResourceMonitorError> {
        let sandbox_path = self.base_path.join("sandbox").join(ai_id);
        
        // Créer le répertoire sandbox s'il n'existe pas
        if !sandbox_path.exists() {
            fs::create_dir_all(&sandbox_path)
                .map_err(|e| ResourceMonitorError::ConfigurationError(
                    format!("Impossible de créer le sandbox pour l'IA {}: {}", ai_id, e)
                ))?;
                
            info!("Répertoire sandbox créé pour l'IA {}: {}", ai_id, sandbox_path.display());
        }
        
        // Enregistrer le chemin
        self.sandbox_paths.insert(ai_id.to_string(), sandbox_path.clone());
        
        Ok(sandbox_path)
    }
    
    /// Vérifie si un chemin est dans le sandbox d'une IA spécifique
    pub fn is_path_in_sandbox(&self, ai_id: &str, path: &Path) -> bool {
        if let Some(sandbox_path) = self.sandbox_paths.get(ai_id) {
            path.starts_with(sandbox_path)
        } else {
            false
        }
    }
    
    /// Obtient le chemin d'un sandbox
    pub fn get_sandbox_path(&self, ai_id: &str) -> Option<&PathBuf> {
        self.sandbox_paths.get(ai_id)
    }
}
