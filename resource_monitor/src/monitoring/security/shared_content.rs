use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use log::info;

use crate::error::ResourceMonitorError;
use crate::security::file_guard::FileGuard;

#[derive(Clone)]
pub struct SharedContentMonitor {
    file_guard: Arc<Mutex<FileGuard>>,
}

impl SharedContentMonitor {
    pub fn new(file_guard: Arc<Mutex<FileGuard>>) -> Self {
        Self { file_guard }
    }
    
    /// Configure la surveillance des fichiers partagés
    pub fn configure_monitoring(&self, shared_path: &Path) -> Result<(), ResourceMonitorError> {
        // Parcourir récursivement le répertoire shared_center
        self.scan_for_binaries(shared_path)?;
        Ok(())
    }
    
    /// Vérifie les modifications dans le répertoire partagé
    pub fn check_modifications(&self, path: &Path) -> Result<Vec<String>, ResourceMonitorError> {
        let path_str = path.to_string_lossy().to_string();
        let mut guard = self.file_guard.lock().unwrap();
        
        // S'assurer que le répertoire est surveillé
        if !guard.is_monitoring_directory(&path_str) {
            guard.add_monitored_directory(&path_str)?;
        }
        
        // Vérifier les modifications
        guard.check_for_modifications()
    }
    
    /// Recherche récursivement les fichiers binaires (.bin) à surveiller
    fn scan_for_binaries(&self, dir: &Path) -> Result<(), ResourceMonitorError> {
        if !dir.is_dir() {
            return Ok(());
        }
        
        let entries = std::fs::read_dir(dir)
            .map_err(|e| ResourceMonitorError::MonitoringError(
                format!("Impossible de lire le répertoire {}: {}", dir.display(), e)
            ))?;
        
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                if path.is_dir() {
                    self.scan_for_binaries(&path)?;
                } else if let Some(ext) = path.extension() {
                    // Surveillance particulière des fichiers .bin (binaires)
                    if ext == "bin" {
                        let path_str = path.to_string_lossy().to_string();
                        
                        // Ajouter à la surveillance intensive
                        let mut guard = self.file_guard.lock().unwrap();
                        guard.add_monitored_file_with_checksum(&path_str)?;
                        
                        info!("Fichier binaire sous surveillance intensive: {}", path_str);
                    }
                    // Surveillance standard des fichiers .ron (lisibles)
                    else if ext == "ron" {
                        let path_str = path.to_string_lossy().to_string();
                        
                        // Surveillance standard
                        let mut guard = self.file_guard.lock().unwrap();
                        guard.add_monitored_file(&path_str)?;
                    }
                }
            }
        }
        
        Ok(())
    }
}
