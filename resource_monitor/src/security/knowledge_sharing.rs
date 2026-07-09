use std::fs;
use std::path::{Path, PathBuf};
use log::{info, warn, error};

use crate::error::ResourceMonitorError;
use crate::security::security_zones::SecurityZoneManager;

/// Gestionnaire pour le partage de connaissances entre IA
pub struct KnowledgeSharing {
    shared_center_path: PathBuf,
}

impl KnowledgeSharing {
    pub fn new(shared_center_path: PathBuf) -> Self {
        Self { shared_center_path }
    }
    
    /// Crée un sous-dossier pour une IA dans le centre de partage
    pub fn create_ai_folder(&self, ai_id: &str) -> Result<PathBuf, ResourceMonitorError> {
        let ai_folder = self.shared_center_path.join(ai_id);
        
        if !ai_folder.exists() {
            fs::create_dir_all(&ai_folder)
                .map_err(|e| ResourceMonitorError::ConfigurationError(
                    format!("Impossible de créer le dossier partagé pour l'IA {}: {}", ai_id, e)
                ))?;
            
            info!("Dossier de partage créé pour l'IA {}: {}", ai_id, ai_folder.display());
        }
        
        Ok(ai_folder)
    }
    
    /// Partage un graphe de connaissances (format RON)
    pub fn share_knowledge_graph(&self, ai_id: &str, name: &str, graph_content: &str) -> Result<PathBuf, ResourceMonitorError> {
        let ai_folder = self.create_ai_folder(ai_id)?;
        let file_path = ai_folder.join(format!("{}.ron", name));
        
        fs::write(&file_path, graph_content)
            .map_err(|e| ResourceMonitorError::ConfigurationError(
                format!("Impossible d'écrire le graphe de connaissances: {}", e)
            ))?;
        
        info!("Graphe de connaissances '{}' partagé par l'IA {}", name, ai_id);
        Ok(file_path)
    }
    
    /// Partage des données binaires
    pub fn share_binary_data(&self, ai_id: &str, name: &str, data: &[u8]) -> Result<PathBuf, ResourceMonitorError> {
        let ai_folder = self.create_ai_folder(ai_id)?;
        let file_path = ai_folder.join(format!("{}.bin", name));
        
        fs::write(&file_path, data)
            .map_err(|e| ResourceMonitorError::ConfigurationError(
                format!("Impossible d'écrire les données binaires: {}", e)
            ))?;
        
        info!("Données binaires '{}' partagées par l'IA {}", name, ai_id);
        Ok(file_path)
    }
    
    /// Liste tous les graphes de connaissances disponibles (.ron)
    pub fn list_available_graphs(&self) -> Result<Vec<(String, String, PathBuf)>, ResourceMonitorError> {
        let mut results = Vec::new();
        
        // Parcourir tous les sous-dossiers d'IA
        if let Ok(entries) = fs::read_dir(&self.shared_center_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(ai_id) = path.file_name().and_then(|n| n.to_str()) {
                            // Rechercher les fichiers .ron dans ce dossier
                            if let Ok(files) = fs::read_dir(&path) {
                                for file in files {
                                    if let Ok(file) = file {
                                        let file_path = file.path();
                                        if file_path.extension().and_then(|ext| ext.to_str()) == Some("ron") {
                                            if let Some(name) = file_path.file_stem().and_then(|n| n.to_str()) {
                                                results.push((ai_id.to_string(), name.to_string(), file_path));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Liste toutes les données binaires disponibles (.bin)
    pub fn list_available_binary_data(&self) -> Result<Vec<(String, String, PathBuf)>, ResourceMonitorError> {
        let mut results = Vec::new();
        
        // Parcourir tous les sous-dossiers d'IA
        if let Ok(entries) = fs::read_dir(&self.shared_center_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        if let Some(ai_id) = path.file_name().and_then(|n| n.to_str()) {
                            // Rechercher les fichiers .bin dans ce dossier
                            if let Ok(files) = fs::read_dir(&path) {
                                for file in files {
                                    if let Ok(file) = file {
                                        let file_path = file.path();
                                        if file_path.extension().and_then(|ext| ext.to_str()) == Some("bin") {
                                            if let Some(name) = file_path.file_stem().and_then(|n| n.to_str()) {
                                                results.push((ai_id.to_string(), name.to_string(), file_path));
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }
    
    /// Charge un graphe de connaissances partagé
    pub fn load_knowledge_graph(&self, ai_id: &str, name: &str) -> Result<String, ResourceMonitorError> {
        let file_path = self.shared_center_path.join(ai_id).join(format!("{}.ron", name));
        
        fs::read_to_string(&file_path)
            .map_err(|e| ResourceMonitorError::ConfigurationError(
                format!("Impossible de lire le graphe de connaissances: {}", e)
            ))
    }
    
    /// Charge des données binaires partagées
    pub fn load_binary_data(&self, ai_id: &str, name: &str) -> Result<Vec<u8>, ResourceMonitorError> {
        let file_path = self.shared_center_path.join(ai_id).join(format!("{}.bin", name));
        
        fs::read(&file_path)
            .map_err(|e| ResourceMonitorError::ConfigurationError(
                format!("Impossible de lire les données binaires: {}", e)
            ))
    }
}
