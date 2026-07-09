use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::path::Path;
use log::{info, warn, error};
use sysinfo::System;

use crate::error::ResourceMonitorError;
use crate::security::file_guard::FileGuard;

#[derive(Clone)]
pub struct FileMonitor {
    system: Arc<Mutex<System>>,
    monitored_directories: HashSet<String>,
    forbidden_paths: HashSet<String>,
    file_guard: Arc<Mutex<FileGuard>>,
}

impl FileMonitor {
    pub fn new(system: Arc<Mutex<System>>, file_guard: Arc<Mutex<FileGuard>>) -> Self {
        let mut forbidden_paths = HashSet::new();
        // Protéger le répertoire du moniteur de ressources
        forbidden_paths.insert("c:\\wamp64\\www\\rust\\multi_ai\\resource_monitor".to_string());
        // Protéger les fichiers système critiques
        forbidden_paths.insert("C:\\Windows\\System32".to_string());
        forbidden_paths.insert("/etc".to_string());
        forbidden_paths.insert("/bin".to_string());
        forbidden_paths.insert("/sbin".to_string());
        
        Self {
            system,
            monitored_directories: HashSet::new(),
            forbidden_paths,
            file_guard,
        }
    }
    
    pub fn add_monitored_directory(&mut self, path: &str) -> Result<(), ResourceMonitorError> {
        let path_str = path.to_string();
        if !Path::new(&path_str).exists() {
            return Err(ResourceMonitorError::ConfigurationError(
                format!("Le répertoire {} n'existe pas", path)
            ));
        }
        
        self.monitored_directories.insert(path_str.clone());
        
        // Aussi ajouter au file guard
        let mut guard = self.file_guard.lock().unwrap();
        guard.add_monitored_directory(&path_str)?;
        
        info!("Répertoire ajouté à la surveillance: {}", path);
        Ok(())
    }
    
    pub fn add_forbidden_path(&mut self, path: &str) {
        self.forbidden_paths.insert(path.to_string());
        
        // Aussi ajouter au file guard
        let mut guard = self.file_guard.lock().unwrap();
        guard.add_forbidden_path(path);
        
        info!("Chemin interdit ajouté: {}", path);
    }
    
    /// Vérifie les modifications non autorisées dans les répertoires surveillés
    pub fn check_for_unauthorized_changes(&self) -> Result<Vec<String>, ResourceMonitorError> {
        let mut guard = self.file_guard.lock().unwrap();
        let changes = guard.check_for_modifications()?;
        
        if !changes.is_empty() {
            warn!("Modifications non autorisées détectées: {:?}", changes);
        }
        
        Ok(changes)
    }
    
    /// Vérifie si un chemin interdit a été accédé
    pub fn check_forbidden_paths(&self) -> Result<bool, ResourceMonitorError> {
        for path in &self.forbidden_paths {
            let path_obj = Path::new(path);
            if path_obj.exists() {
                let mut guard = self.file_guard.lock().unwrap();
                if let Ok(true) = guard.has_recent_access(path) {
                    error!("ALERTE CRITIQUE: Accès non autorisé détecté à {}", path);
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}
