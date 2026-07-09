use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, Duration};
use log::{info, warn, error};

use crate::error::ResourceMonitorError;
use crate::security::cryptography::CryptographyProvider;

/// Garde de fichiers pour prévenir les modifications non autorisées
pub struct FileGuard {
    monitored_files: HashMap<String, SystemTime>,
    monitored_directories: HashSet<String>,
    forbidden_paths: HashSet<String>,
    last_check_time: SystemTime,
    // Nouveau: stockage des checksums pour les fichiers binaires
    file_checksums: HashMap<String, Vec<u8>>,
    // Nouveau: provider de crypto pour génération de checksums
    crypto_provider: Option<CryptographyProvider>,
}

impl FileGuard {
    pub fn new() -> Self {
        // Initialiser le crypto provider avec une clé par défaut
        let crypto = CryptographyProvider::new(b"resource_monitor_default_key").ok();
        
        Self {
            monitored_files: HashMap::new(),
            monitored_directories: HashSet::new(),
            forbidden_paths: HashSet::new(),
            last_check_time: SystemTime::now(),
            file_checksums: HashMap::new(),
            crypto_provider: crypto,
        }
    }
    
    /// Ajoute un fichier à surveiller
    pub fn add_monitored_file(&mut self, path: &str) -> Result<(), ResourceMonitorError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() || !path_obj.is_file() {
            return Err(ResourceMonitorError::ConfigurationError(
                format!("Le fichier {} n'existe pas", path)
            ));
        }
        
        // Obtenir la date de dernière modification
        if let Ok(metadata) = fs::metadata(path_obj) {
            if let Ok(modified) = metadata.modified() {
                self.monitored_files.insert(path.to_string(), modified);
                info!("Fichier ajouté à la surveillance: {}", path);
                return Ok(());
            }
        }
        
        Err(ResourceMonitorError::MonitoringError(
            format!("Impossible d'obtenir les métadonnées du fichier {}", path)
        ))
    }
    
    /// Ajoute un répertoire à surveiller
    pub fn add_monitored_directory(&mut self, path: &str) -> Result<(), ResourceMonitorError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() || !path_obj.is_dir() {
            return Err(ResourceMonitorError::ConfigurationError(
                format!("Le répertoire {} n'existe pas", path)
            ));
        }
        
        self.monitored_directories.insert(path.to_string());
        info!("Répertoire ajouté à la surveillance: {}", path);
        
        // Indexer tous les fichiers du répertoire
        self.index_directory(path_obj)?;
        
        Ok(())
    }
    
    /// Ajoute un chemin interdit
    pub fn add_forbidden_path(&mut self, path: &str) {
        self.forbidden_paths.insert(path.to_string());
        info!("Chemin interdit ajouté: {}", path);
    }
    
    /// Indexe récursivement tous les fichiers d'un répertoire
    fn index_directory(&mut self, dir_path: &Path) -> Result<(), ResourceMonitorError> {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Récursion pour les sous-répertoires
                        self.index_directory(&path)?;
                    } else {
                        // Ajouter le fichier à la surveillance
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                self.monitored_files.insert(path.to_string_lossy().to_string(), modified);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Vérifie si un répertoire est déjà surveillé
    pub fn is_monitoring_directory(&self, path: &str) -> bool {
        self.monitored_directories.contains(path)
    }
    
    /// Ajoute un fichier à surveiller avec vérification de checksum (pour les fichiers binaires)
    pub fn add_monitored_file_with_checksum(&mut self, path: &str) -> Result<(), ResourceMonitorError> {
        // D'abord ajouter à la surveillance normale
        self.add_monitored_file(path)?;
        
        // Puis calculer et stocker le checksum
        if let Some(crypto) = &self.crypto_provider {
            let path_obj = Path::new(path);
            if let Ok(content) = fs::read(path_obj) {
                let checksum = crypto.hash(&content);
                self.file_checksums.insert(path.to_string(), checksum);
                info!("Checksum calculé et stocké pour le fichier: {}", path);
            } else {
                return Err(ResourceMonitorError::MonitoringError(
                    format!("Impossible de lire le contenu du fichier {} pour calculer le checksum", path)
                ));
            }
        } else {
            warn!("Crypto provider non disponible, surveillance sans checksum pour: {}", path);
        }
        
        Ok(())
    }
    
    /// Vérifie les checksums des fichiers binaires
    fn verify_file_checksums(&self) -> Vec<String> {
        let mut modified_files = Vec::new();
        
        if let Some(crypto) = &self.crypto_provider {
            for (path, stored_checksum) in &self.file_checksums {
                let path_obj = Path::new(path);
                if path_obj.exists() {
                    if let Ok(content) = fs::read(path_obj) {
                        let current_checksum = crypto.hash(&content);
                        
                        if &current_checksum != stored_checksum {
                            warn!("Modification détectée dans le fichier binaire: {}", path);
                            modified_files.push(path.clone());
                        }
                    } else {
                        warn!("Impossible de lire le contenu du fichier binaire: {}", path);
                    }
                } else {
                    warn!("Fichier binaire supprimé: {}", path);
                    modified_files.push(path.clone());
                }
            }
        }
        
        modified_files
    }
    
    /// Vérifie si des fichiers surveillés ont été modifiés (version étendue)
    pub fn check_for_modifications(&mut self) -> Result<Vec<String>, ResourceMonitorError> {
        let mut modified_files = Vec::new();
        
        // Vérifier les fichiers surveillés individuellement
        for (path, last_modified) in &self.monitored_files {
            let path_obj = Path::new(path);
            if path_obj.exists() {
                if let Ok(metadata) = fs::metadata(path_obj) {
                    if let Ok(current_modified) = metadata.modified() {
                        if &current_modified > last_modified {
                            modified_files.push(path.clone());
                            warn!("Fichier modifié détecté: {}", path);
                        }
                    }
                } else {
                    // Le fichier a été supprimé
                    modified_files.push(path.clone());
                    warn!("Fichier supprimé détecté: {}", path);
                }
            }
        }
        
        // Vérifier si des accès aux chemins interdits ont été tentés
        for path in &self.forbidden_paths {
            let path_obj = Path::new(path);
            if path_obj.exists() {
                if let Ok(metadata) = fs::metadata(path_obj) {
                    if let Ok(current_modified) = metadata.modified() {
                        // Vérifier si modifié depuis le dernier contrôle
                        if let Ok(duration) = current_modified.duration_since(self.last_check_time) {
                            if duration < Duration::from_secs(300) { // 5 minutes
                                error!("ALERTE: Tentative d'accès à un chemin interdit: {}", path);
                                modified_files.push(path.clone());
                            }
                        }
                    }
                }
            }
        }
        
        // Vérifier également les checksums des fichiers binaires
        let binary_modifications = self.verify_file_checksums();
        modified_files.extend(binary_modifications);
        
        self.last_check_time = SystemTime::now();
        Ok(modified_files)
    }
    
    /// Vérifie si un fichier a été accédé récemment
    pub fn has_recent_access(&self, path: &str) -> Result<bool, ResourceMonitorError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(ResourceMonitorError::MonitoringError(
                format!("Le chemin {} n'existe pas", path)
            ));
        }
        
        if let Ok(metadata) = fs::metadata(path_obj) {
            if let Ok(accessed) = metadata.accessed() {
                // Vérifier si l'accès est récent (moins de 5 minutes)
                if let Ok(duration) = SystemTime::now().duration_since(accessed) {
                    return Ok(duration < Duration::from_secs(300));
                }
            }
        }
        
        Ok(false)
    }
}
