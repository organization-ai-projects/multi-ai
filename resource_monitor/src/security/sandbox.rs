use std::process::Command;
use log::{info, warn, error};
use std::path::Path;

use crate::error::ResourceMonitorError;

/// Gestionnaire d'isolation des IA pour éviter qu'elles ne modifient le système
pub struct Sandbox {
    enabled: bool,
    isolation_level: IsolationLevel,
}

/// Niveau d'isolation pour les IA
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IsolationLevel {
    /// Isolation minimale (surveillance uniquement)
    Basic,
    /// Isolation moyenne (limitation des ressources et accès)
    Standard,
    /// Isolation maximale (conteneurisation complète)
    Container,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            enabled: true,
            isolation_level: IsolationLevel::Standard,
        }
    }
    
    /// Démarre une IA dans un environnement isolé
    pub fn launch_isolated_ai(&self, ai_path: &str, args: &[&str]) -> Result<u32, ResourceMonitorError> {
        if !self.enabled {
            warn!("Sandbox désactivée! L'IA sera lancée sans isolation.");
            return self.launch_process(ai_path, args);
        }
        
        match self.isolation_level {
            IsolationLevel::Basic => {
                info!("Lancement de l'IA {} avec isolation basique", ai_path);
                self.launch_process(ai_path, args)
            },
            IsolationLevel::Standard => {
                info!("Lancement de l'IA {} avec isolation standard", ai_path);
                self.launch_with_restricted_permissions(ai_path, args)
            },
            IsolationLevel::Container => {
                info!("Lancement de l'IA {} dans un conteneur", ai_path);
                self.launch_in_container(ai_path, args)
            },
        }
    }
    
    /// Lance un processus normal sans isolation
    fn launch_process(&self, program: &str, args: &[&str]) -> Result<u32, ResourceMonitorError> {
        let mut cmd = Command::new(program);
        cmd.args(args);
        
        let child = cmd.spawn()
            .map_err(|e| ResourceMonitorError::MonitoringError(
                format!("Erreur lors du lancement du processus: {}", e)
            ))?;
        
        Ok(child.id())
    }
    
    /// Lance un processus avec des permissions restreintes
    fn launch_with_restricted_permissions(&self, program: &str, args: &[&str]) -> Result<u32, ResourceMonitorError> {
        #[cfg(target_os = "windows")]
        {
            // Sur Windows, on pourrait utiliser JobObjects pour restreindre les ressources
            // Ou CreateRestrictedToken pour limiter les permissions
            // Pour l'exemple, on lance juste le processus normalement
            warn!("Isolation limitée sur Windows: lancement sans restrictions réelles");
            self.launch_process(program, args)
        }
        
        #[cfg(unix)]
        {
            // Sur Unix, on peut utiliser chroot, namespaces, ou simplement des utilisateurs restreints
            
            // Option 1: Lancer avec un utilisateur non privilégié
            let mut cmd = Command::new("sudo");
            cmd.args(&["-u", "nobody", program]);
            cmd.args(args);
            
            let child = cmd.spawn()
                .map_err(|e| ResourceMonitorError::MonitoringError(
                    format!("Erreur lors du lancement du processus restreint: {}", e)
                ))?;
            
            Ok(child.id())
        }
    }
    
    /// Lance un processus dans un conteneur
    fn launch_in_container(&self, program: &str, args: &[&str]) -> Result<u32, ResourceMonitorError> {
        if which::which("docker").is_err() {
            return Err(ResourceMonitorError::ConfigurationError(
                "Docker n'est pas installé pour l'isolation par conteneur".to_string()
            ));
        }
        
        // Construire le chemin absolu
        let abs_path = Path::new(program).canonicalize()
            .map_err(|e| ResourceMonitorError::ConfigurationError(
                format!("Impossible de résoudre le chemin de l'IA: {}", e)
            ))?;
        
        let dir = abs_path.parent()
            .ok_or_else(|| ResourceMonitorError::ConfigurationError(
                "Impossible de déterminer le répertoire parent".to_string()
            ))?;
        
        let filename = abs_path.file_name()
            .ok_or_else(|| ResourceMonitorError::ConfigurationError(
                "Impossible de déterminer le nom de fichier".to_string()
            ))?;
        
        // Lancer Docker avec des limites strictes
        let mut cmd = Command::new("docker");
        cmd.args(&["run", "--rm", 
                  "--memory=2g", "--cpus=1", 
                  "-v", &format!("{}:/app", dir.to_string_lossy()),
                  "--workdir=/app",
                  "alpine:latest", 
                  &format!("/app/{}", filename.to_string_lossy())]);
        
        // Ajouter les arguments du programme
        cmd.args(args);
        
        let output = cmd.output()
            .map_err(|e| ResourceMonitorError::MonitoringError(
                format!("Erreur lors du lancement du conteneur: {}", e)
            ))?;
        
        // Dans une implémentation réelle, nous devrions récupérer le PID du conteneur
        // Pour l'exemple, on renvoie juste un ID fictif
        Ok(9999)
    }
    
    /// Définit le niveau d'isolation
    pub fn set_isolation_level(&mut self, level: IsolationLevel) {
        info!("Niveau d'isolation modifié: {:?}", level);
        self.isolation_level = level;
    }
    
    /// Active ou désactive le sandbox
    pub fn set_enabled(&mut self, enabled: bool) {
        if !enabled {
            warn!("⚠️ ATTENTION: Désactivation du sandbox - les IA ne seront plus isolées!");
        } else {
            info!("Sandbox activé");
        }
        self.enabled = enabled;
    }
}