use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use log::{info, warn, error};

use crate::error::ResourceMonitorError;

/// Définit les différents niveaux d'accès pour chaque zone
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccessLevel {
    /// Accès complet (lecture/écriture/exécution) - pour sandbox de l'IA
    FullAccess,
    /// Accès en lecture/écriture (pour shared_center)
    ReadWrite,
    /// Accès en lecture seule
    ReadOnly,
    /// Accès interdit
    Denied,
}

/// Gère les différentes zones de sécurité et leurs permissions
#[derive(Clone)]
pub struct SecurityZoneManager {
    base_path: PathBuf,
    ai_sandboxes: HashMap<String, PathBuf>,
    shared_center: PathBuf,
    ai_permissions: HashMap<String, Vec<(PathBuf, AccessLevel)>>,
}

impl SecurityZoneManager {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        let base = base_path.as_ref().to_path_buf();
        
        // Initialiser le centre de partage commun (unique à toutes les IA)
        let shared_center = base.join("shared_center");
        
        // Créer les répertoires s'ils n'existent pas
        if !shared_center.exists() {
            if let Err(e) = fs::create_dir_all(&shared_center) {
                error!("Impossible de créer le répertoire shared_center: {}", e);
            } else {
                info!("Répertoire shared_center créé: {}", shared_center.display());
            }
        }
        
        // Créer le répertoire sandbox s'il n'existe pas
        let sandbox_dir = base.join("sandbox");
        if !sandbox_dir.exists() {
            if let Err(e) = fs::create_dir_all(&sandbox_dir) {
                error!("Impossible de créer le répertoire sandbox: {}", e);
            } else {
                info!("Répertoire sandbox créé: {}", sandbox_dir.display());
            }
        }
        
        Self {
            base_path: base,
            ai_sandboxes: HashMap::new(),
            shared_center,
            ai_permissions: HashMap::new(),
        }
    }
    
    /// Enregistre une IA et crée son sandbox dédié
    pub fn register_ai(&mut self, ai_id: &str) -> Result<(), ResourceMonitorError> {
        // Créer le sandbox individuel pour l'IA
        let sandbox_path = self.base_path.join("sandbox").join(ai_id);
        
        // Créer le répertoire sandbox de l'IA s'il n'existe pas
        if !sandbox_path.exists() {
            fs::create_dir_all(&sandbox_path)
                .map_err(|e| ResourceMonitorError::ConfigurationError(
                    format!("Impossible de créer le sandbox pour l'IA {}: {}", ai_id, e)
                ))?;
        }
        
        // Enregistrer le sandbox
        self.ai_sandboxes.insert(ai_id.to_string(), sandbox_path.clone());
        
        // Créer aussi son répertoire dans le centre de partage commun
        let shared_ai_dir = self.shared_center.join(ai_id);
        if !shared_ai_dir.exists() {
            fs::create_dir_all(&shared_ai_dir)
                .map_err(|e| ResourceMonitorError::ConfigurationError(
                    format!("Impossible de créer le dossier de l'IA dans le centre de partage: {}", e)
                ))?;
        }
        
        // Configurer les permissions par défaut pour cette IA
        let mut permissions = Vec::new();
        
        // Accès complet à son propre sandbox
        permissions.push((sandbox_path, AccessLevel::FullAccess));
        
        // Accès en lecture/écriture à son propre dossier dans le centre de partage
        permissions.push((shared_ai_dir, AccessLevel::ReadWrite));
        
        // Accès en lecture seule au reste du centre de partage commun
        permissions.push((self.shared_center.clone(), AccessLevel::ReadOnly));
        
        // Enregistrer les permissions
        self.ai_permissions.insert(ai_id.to_string(), permissions);
        
        info!("IA {} enregistrée avec son sandbox et accès au centre de partage", ai_id);
        Ok(())
    }
    
    /// Vérifie si une IA a accès à un chemin spécifié avec le niveau d'accès requis
    pub fn check_access(&self, ai_id: &str, path: &Path, required_access: AccessLevel) -> bool {
        // Récupérer les permissions de l'IA
        if let Some(permissions) = self.ai_permissions.get(ai_id) {
            // Vérifier si le chemin est couvert par une des règles de permission
            for (allowed_path, access_level) in permissions {
                if path.starts_with(allowed_path) {
                    // Si le chemin est autorisé, vérifier le niveau d'accès
                    match (*access_level, required_access) {
                        // Accès complet permet tout
                        (AccessLevel::FullAccess, _) => return true,
                        // ReadWrite permet ReadWrite et ReadOnly
                        (AccessLevel::ReadWrite, AccessLevel::ReadWrite | AccessLevel::ReadOnly) => return true,
                        // ReadOnly permet uniquement ReadOnly
                        (AccessLevel::ReadOnly, AccessLevel::ReadOnly) => return true,
                        // Autres combinaisons refusées
                        _ => return false,
                    }
                }
            }
        }
        
        // Par défaut, accès refusé
        false
    }
    
    /// Retourne le chemin du sandbox d'une IA
    pub fn get_ai_sandbox(&self, ai_id: &str) -> Option<&Path> {
        self.ai_sandboxes.get(ai_id).map(|p| p.as_path())
    }
    
    /// Retourne le chemin du centre de partage
    pub fn get_shared_center(&self) -> &Path {
        &self.shared_center
    }
    
    /// Modifie les permissions d'accès d'une IA pour un chemin spécifique
    pub fn set_ai_permission(&mut self, ai_id: &str, path: &Path, access: AccessLevel) -> Result<(), ResourceMonitorError> {
        let permissions = self.ai_permissions
            .entry(ai_id.to_string())
            .or_insert_with(Vec::new);
        
        // Chercher si une règle existe déjà pour ce chemin
        for permission in permissions.iter_mut() {
            if permission.0 == path {
                // Mettre à jour l'accès
                permission.1 = access;
                return Ok(());
            }
        }
        
        // Ajouter une nouvelle règle
        permissions.push((path.to_path_buf(), access));
        Ok(())
    }
    
    /// Vérifie si un fichier est dans une zone partagée
    pub fn is_in_shared_center(&self, path: &Path) -> bool {
        path.starts_with(&self.shared_center)
    }
    
    /// Vérifie si un fichier est dans un sandbox d'IA
    pub fn is_in_sandbox(&self, path: &Path) -> Option<String> {
        let sandbox_root = self.base_path.join("sandbox");
        
        if path.starts_with(&sandbox_root) {
            // Extraire le nom du dossier sandbox (nom de l'IA)
            if let Some(relative) = path.strip_prefix(&sandbox_root).ok() {
                if let Some(ai_name) = relative.components().next() {
                    return Some(ai_name.as_os_str().to_string_lossy().to_string());
                }
            }
        }
        
        None
    }
    
    /// Crée un sous-dossier pour une IA dans le centre de partage
    pub fn create_ai_shared_folder(&self, ai_id: &str) -> Result<PathBuf, ResourceMonitorError> {
        let ai_shared = self.shared_center.join(ai_id);
        
        if !ai_shared.exists() {
            fs::create_dir_all(&ai_shared)
                .map_err(|e| ResourceMonitorError::ConfigurationError(
                    format!("Impossible de créer le dossier partagé pour l'IA {}: {}", ai_id, e)
                ))?;
        }
        
        Ok(ai_shared)
    }
}
