use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use log::{info, warn, error};
use sysinfo::System;

use crate::models::security::SecurityLevel;
use crate::error::ResourceMonitorError;
use crate::security::file_guard::FileGuard;
use crate::security::security_zones::{SecurityZoneManager, AccessLevel};

#[derive(Clone)]
pub struct SecurityManager {
    system: Arc<Mutex<System>>,
    security_level: SecurityLevel,
    monitored_directories: HashSet<String>,
    forbidden_paths: HashSet<String>,
    file_guard: Arc<Mutex<FileGuard>>,
    // Nouveau: ajout du gestionnaire de zones de sécurité
    zone_manager: Arc<Mutex<SecurityZoneManager>>,
}

impl SecurityManager {
    pub fn new(system: Arc<Mutex<System>>) -> Self {
        let mut forbidden_paths = HashSet::new();
        // Protéger le répertoire du moniteur de ressources
        forbidden_paths.insert("c:\\wamp64\\www\\rust\\multi_ai\\resource_monitor".to_string());
        // Protéger les fichiers système critiques
        forbidden_paths.insert("C:\\Windows\\System32".to_string());
        forbidden_paths.insert("/etc".to_string());
        forbidden_paths.insert("/bin".to_string());
        forbidden_paths.insert("/sbin".to_string());
        
        // Créer le gestionnaire de zones avec le chemin de base
        let zone_manager = SecurityZoneManager::new("c:\\wamp64\\www\\rust\\multi_ai");
        
        Self {
            system,
            security_level: SecurityLevel::High,
            monitored_directories: HashSet::new(),
            forbidden_paths,
            file_guard: Arc::new(Mutex::new(FileGuard::new())),
            zone_manager: Arc::new(Mutex::new(zone_manager)),
        }
    }
    
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        info!("Niveau de sécurité modifié: {:?}", level);
        self.security_level = level;
    }
    
    pub fn get_security_level(&self) -> SecurityLevel {
        self.security_level.clone()
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
    
    /// Enregistre une IA dans le système de zones de sécurité
    pub fn register_ai_in_zones(&self, ai_id: &str) -> Result<(), ResourceMonitorError> {
        let mut zones = self.zone_manager.lock().unwrap();
        zones.register_ai(ai_id)
    }
    
    /// Vérifie si une IA a le droit d'accéder à un fichier
    pub fn check_file_access(&self, ai_id: &str, path: &str, write_access: bool) -> bool {
        let zones = self.zone_manager.lock().unwrap();
        
        let path_obj = Path::new(path);
        
        // Déterminer le niveau d'accès requis
        let required_access = if write_access {
            AccessLevel::ReadWrite
        } else {
            AccessLevel::ReadOnly
        };
        
        zones.check_access(ai_id, path_obj, required_access)
    }
    
    /// Vérifie si un fichier est dans une zone partagée
    pub fn is_in_shared_zone(&self, path: &str) -> bool {
        let zones = self.zone_manager.lock().unwrap();
        let path_obj = Path::new(path);
        
        zones.is_in_shared_center(path_obj)
    }
    
    /// Vérifie si un fichier est dans un sandbox d'IA
    pub fn get_sandbox_owner(&self, path: &str) -> Option<String> {
        let zones = self.zone_manager.lock().unwrap();
        let path_obj = Path::new(path);
        
        zones.is_in_sandbox(path_obj)
    }
    
    /// Obtient le chemin du sandbox d'une IA
    pub fn get_ai_sandbox_path(&self, ai_id: &str) -> Option<PathBuf> {
        let zones = self.zone_manager.lock().unwrap();
        zones.get_ai_sandbox(ai_id).map(|p| p.to_path_buf())
    }
    
    /// Obtient le chemin du centre de partage
    pub fn get_shared_center_path(&self) -> PathBuf {
        let zones = self.zone_manager.lock().unwrap();
        zones.get_shared_center().to_path_buf()
    }
    
    /// Vérifie les modifications dans les zones partagées
    pub fn check_shared_modifications(&self) -> Result<Vec<String>, ResourceMonitorError> {
        let path = self.get_shared_center_path().to_string_lossy().to_string();
        let mut guard = self.file_guard.lock().unwrap();
        
        // S'assurer que le répertoire est surveillé
        if !guard.is_monitoring_directory(&path) {
            guard.add_monitored_directory(&path)?;
        }
        
        // Vérifier les modifications
        guard.check_for_modifications()
    }
    
    /// Configure la surveillance spécifique des fichiers binaires (.bin) dans le centre de partage
    pub fn configure_shared_binary_monitoring(&self) -> Result<(), ResourceMonitorError> {
        let shared_path = self.get_shared_center_path();
        
        // Parcourir récursivement le répertoire shared_center
        self.scan_for_binaries(&shared_path)?;
        
        Ok(())
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
