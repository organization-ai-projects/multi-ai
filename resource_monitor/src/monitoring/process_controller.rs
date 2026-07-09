use std::process::Command;
use std::fs;
use log::{warn, info, error};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::error::ResourceMonitorError;

#[derive(Clone)]
pub struct ProcessController {
    // Ajout d'un répertoire pour les fichiers de signaux
    signal_dir: String,
    // Ajout de privilèges élevés pour le contrôleur
    has_elevated_privileges: bool,
    // Nouveau: Flag pour activer le mode d'urgence
    emergency_mode: Arc<AtomicBool>,
}

impl ProcessController {
    pub fn new() -> Self {
        Self {
            signal_dir: ".".to_string(),
            has_elevated_privileges: Self::check_elevated_privileges(),
            emergency_mode: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn with_signal_dir(mut self, dir: &str) -> Self {
        self.signal_dir = dir.to_string();
        self
    }
    
    /// Active le mode d'urgence qui impose des restrictions plus sévères
    pub fn enable_emergency_mode(&self) {
        warn!("⚠️ MODE D'URGENCE ACTIVÉ - Restrictions de sécurité maximales");
        self.emergency_mode.store(true, Ordering::SeqCst);
    }
    
    /// Désactive le mode d'urgence
    pub fn disable_emergency_mode(&self) {
        info!("Mode d'urgence désactivé - Retour aux restrictions normales");
        self.emergency_mode.store(false, Ordering::SeqCst);
    }
    
    /// Vérifie si le mode d'urgence est actif
    pub fn is_emergency_mode(&self) -> bool {
        self.emergency_mode.load(Ordering::SeqCst)
    }
    
    // NOUVEAU: Remplace les méthodes individuelles par un contrôle centralisé
    /// Contrôle complet d'un processus d'IA (pause, reprise, terminaison)
    pub fn control_ai_process(&self, ai_id: &str, pid: u32, action: AiAction, signal_dir: &Path) -> Result<(), ResourceMonitorError> {
        match action {
            AiAction::Pause => {
                // Créer le signal de pause dans le répertoire de l'IA
                let pause_file = signal_dir.join("ai_pause.signal");
                fs::write(&pause_file, "PAUSE")
                    .map_err(|e| ResourceMonitorError::MonitoringError(
                        format!("Impossible d'écrire le signal de pause: {}", e)
                    ))?;
                
                info!("Signal de pause envoyé à l'IA {} (PID: {})", ai_id, pid);
            },
            AiAction::Resume => {
                // D'abord s'assurer que le mode d'urgence n'est pas actif
                if self.is_emergency_mode() {
                    warn!("Tentative de reprise de l'IA {} bloquée par le mode d'urgence", ai_id);
                    return Err(ResourceMonitorError::SecurityViolation(
                        "Reprise impossible en mode d'urgence".to_string()
                    ));
                }
                
                // Supprimer le signal de pause
                let pause_file = signal_dir.join("ai_pause.signal");
                if pause_file.exists() {
                    fs::remove_file(&pause_file)
                        .map_err(|e| ResourceMonitorError::MonitoringError(
                            format!("Impossible de supprimer le signal de pause: {}", e)
                        ))?;
                }
                
                // Créer le signal de reprise
                let resume_file = signal_dir.join("ai_resume.signal");
                fs::write(&resume_file, "RESUME")
                    .map_err(|e| ResourceMonitorError::MonitoringError(
                        format!("Impossible d'écrire le signal de reprise: {}", e)
                    ))?;
                
                info!("Signal de reprise envoyé à l'IA {} (PID: {})", ai_id, pid);
            },
            AiAction::Terminate => {
                // D'abord essayer un arrêt propre avec un signal
                let terminate_file = signal_dir.join("ai_terminate.signal");
                if let Err(e) = fs::write(&terminate_file, "TERMINATE") {
                    warn!("Impossible d'écrire le signal de terminaison: {}", e);
                }
                
                // Attendre un peu pour laisser l'IA se terminer proprement
                std::thread::sleep(std::time::Duration::from_millis(500));
                
                // Ensuite vérifier si le processus existe toujours
                if Self::is_process_running(pid) {
                    // Si oui, forcer la terminaison
                    self.force_terminate_process(pid)?;
                }
                
                info!("IA {} (PID: {}) terminée", ai_id, pid);
            },
            AiAction::EmergencyKill => {
                // Tuer immédiatement sans attendre
                warn!("ARRÊT D'URGENCE de l'IA {} (PID: {})", ai_id, pid);
                self.force_terminate_process(pid)?;
                
                // Marquer l'incident dans un fichier d'alerte
                let alert_file = signal_dir.join("security_violation.alert");
                fs::write(&alert_file, format!("VIOLATION DE SÉCURITÉ: {}\nTimestamp: {}", 
                                              ai_id, chrono::Local::now()))
                    .map_err(|e| ResourceMonitorError::MonitoringError(
                        format!("Impossible d'écrire le fichier d'alerte: {}", e)
                    ))?;
            }
        }
        
        Ok(())
    }
    
    /// Vérifie si un processus est toujours en cours d'exécution
    fn is_process_running(pid: u32) -> bool {
        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("tasklist")
                .args(&["/FI", &format!("PID eq {}", pid)])
                .output() {
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains(&pid.to_string())
            } else {
                false
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            Path::new(&format!("/proc/{}", pid)).exists()
        }
    }
    
    /// Force la terminaison d'un processus
    fn force_terminate_process(&self, pid: u32) -> Result<(), ResourceMonitorError> {
        #[cfg(target_os = "windows")]
        {
            Command::new("taskkill")
                .args(&["/F", "/PID", &pid.to_string()])
                .output()
                .map_err(|e| ResourceMonitorError::MonitoringError(
                    format!("Impossible de terminer le processus {}: {}", pid, e)
                ))?;
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            Command::new("kill")
                .args(&["-9", &pid.to_string()])
                .output()
                .map_err(|e| ResourceMonitorError::MonitoringError(
                    format!("Impossible de terminer le processus {}: {}", pid, e)
                ))?;
        }
        
        Ok(())
    }
    
    /// Vérifie si le processus a des privilèges élevés
    fn check_elevated_privileges() -> bool {
        #[cfg(target_os = "windows")]
        {
            // Sur Windows, vérifier si le processus est exécuté en tant qu'administrateur
            if let Ok(output) = Command::new("whoami")
                .args(&["/priv"])
                .output() {
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Regarder si on a le privilège SeDebugPrivilege
                return output_str.contains("SeDebugPrivilege");
            }
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            // Sur Unix, vérifier si on est root
            if let Ok(output) = Command::new("id")
                .args(&["-u"])
                .output() {
                
                let output_str = String::from_utf8_lossy(&output.stdout);
                return output_str.trim() == "0"; // 0 = root
            }
        }
        
        false
    }
    
    // Nouvelle méthode pour détecter si une IA supporte le mécanisme de pause par fichier
    pub fn supports_signal_pause(&self, ai_dir: &str) -> bool {
        // Vérifier si le code source de l'IA contient pause_handler.rs
        // Ce n'est pas parfait mais donne une indication
        Path::new(&format!("{}/src/pause_handler.rs", ai_dir)).exists()
    }
}

/// Actions possibles sur une IA
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AiAction {
    Pause,
    Resume,
    Terminate,
    EmergencyKill,
}