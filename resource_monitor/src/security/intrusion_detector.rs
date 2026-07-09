use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{warn, error, info};

use crate::models::security::{SecurityAlert, SecurityLevel};
use crate::error::ResourceMonitorError;

/// Détecte les activités suspectes basées sur des modèles comportementaux
#[derive(Clone)]
pub struct IntrusionDetector {
    // Historique des activités pour détecter des motifs
    activity_log: Arc<Mutex<Vec<ActivityEvent>>>,
    // Compteurs pour détecter les tentatives répétées d'activités suspectes
    attempt_counters: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    // Seuil d'alerte pour les activités suspectes
    alert_threshold: u32,
    // Temps de reset des compteurs
    counter_reset_time: Duration,
    // Niveau de sécurité actuel
    security_level: SecurityLevel,
}

/// Type d'événement d'activité
#[derive(Debug, Clone)]
pub enum ActivityEvent {
    FileModification {
        path: String,
        timestamp: Instant,
    },
    ProcessCreation {
        pid: u32,
        parent_pid: u32,
        timestamp: Instant,
    },
    ResourceSpike {
        resource_type: String, // "cpu", "memory", etc.
        usage_percent: f32,
        timestamp: Instant,
    },
    NetworkAccess {
        destination: String,
        port: u16,
        timestamp: Instant,
    },
}

impl IntrusionDetector {
    pub fn new(security_level: SecurityLevel) -> Self {
        Self {
            activity_log: Arc::new(Mutex::new(Vec::with_capacity(1000))), // Limite pour éviter la croissance infinie
            attempt_counters: Arc::new(Mutex::new(HashMap::new())),
            alert_threshold: match security_level {
                SecurityLevel::Standard => 10,
                SecurityLevel::High => 5,
                SecurityLevel::Critical => 3,
            },
            counter_reset_time: Duration::from_secs(300), // 5 minutes
            security_level,
        }
    }
    
    /// Met à jour le niveau de sécurité
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.security_level = level;
        self.alert_threshold = match level {
            SecurityLevel::Standard => 10,
            SecurityLevel::High => 5,
            SecurityLevel::Critical => 3,
        };
    }
    
    /// Enregistre une activité et retourne true si elle est suspecte
    pub fn log_activity(&self, event: ActivityEvent) -> bool {
        let mut log = self.activity_log.lock().unwrap();
        
        // Garder une taille raisonnable
        if log.len() >= 1000 {
            log.remove(0);
        }
        
        log.push(event.clone());
        
        // Analyser pour détecter des activités suspectes
        match event {
            ActivityEvent::FileModification { path, .. } => {
                self.increment_counter(&format!("file_mod:{}", path))
            },
            ActivityEvent::ProcessCreation { pid, parent_pid, .. } => {
                self.increment_counter(&format!("proc_create:{}", parent_pid))
            },
            ActivityEvent::ResourceSpike { resource_type, usage_percent, .. } => {
                if usage_percent > 90.0 {
                    self.increment_counter(&format!("resource_spike:{}", resource_type))
                } else {
                    false
                }
            },
            ActivityEvent::NetworkAccess { destination, port, .. } => {
                // Ports sensibles ou destinations inhabituelles
                if port < 1024 || destination.contains("unknown") {
                    self.increment_counter(&format!("network_access:{}:{}", destination, port))
                } else {
                    false
                }
            },
        }
    }
    
    /// Incrémente un compteur d'événements et vérifie s'il dépasse le seuil
    fn increment_counter(&self, key: &str) -> bool {
        let mut counters = self.attempt_counters.lock().unwrap();
        
        let now = Instant::now();
        let (count, timestamp) = counters.entry(key.to_string())
            .and_modify(|(count, timestamp)| {
                // Réinitialiser le compteur si trop ancien
                if now.duration_since(*timestamp) > self.counter_reset_time {
                    *count = 1;
                    *timestamp = now;
                } else {
                    *count += 1;
                }
            })
            .or_insert((1, now));
        
        // Vérifier si le seuil est dépassé
        if *count >= self.alert_threshold {
            warn!("Détection d'intrusion: activité suspecte '{}' détectée {} fois", key, count);
            return true;
        }
        
        false
    }
    
    /// Génère une alerte de sécurité basée sur l'activité récente
    pub fn generate_security_alert(&self, suspicious_activity: &str) -> SecurityAlert {
        // Logique pour créer l'alerte appropriée
        if suspicious_activity.starts_with("file_mod:") {
            let path = suspicious_activity.strip_prefix("file_mod:").unwrap_or("");
            SecurityAlert::UnauthorizedFileModification {
                path: path.to_string(),
                modified_by: None,
            }
        } else if suspicious_activity.starts_with("proc_create:") {
            let parent_pid_str = suspicious_activity.strip_prefix("proc_create:").unwrap_or("0");
            let parent_pid = parent_pid_str.parse::<u32>().unwrap_or(0);
            SecurityAlert::ExcessiveProcessCreation {
                ai_id: "unknown".to_string(),
                parent_pid,
                child_count: 0, // À remplir avec la vraie valeur
            }
        } else if suspicious_activity.starts_with("resource_spike:") {
            let resource = suspicious_activity.strip_prefix("resource_spike:").unwrap_or("");
            SecurityAlert::ExcessiveResourceUsage {
                ai_id: "unknown".to_string(),
                pid: 0, // À remplir avec la vraie valeur
                cpu_percent: 0.0, // À remplir avec la vraie valeur
                memory_mb: 0, // À remplir avec la vraie valeur
            }
        } else {
            // Cas par défaut
            SecurityAlert::ForbiddenPathAccess {
                path: suspicious_activity.to_string(),
                ai_id: None,
                pid: None,
            }
        }
    }
    
    /// Analyse les tendances pour détecter les comportements anormaux
    pub fn analyze_trends(&self) -> Vec<String> {
        let log = self.activity_log.lock().unwrap();
        let mut suspicious_patterns = Vec::new();
        
        // Exemple: détecter des séquences d'accès à des fichiers sensibles
        // puis création de processus
        
        // Exemple simplifié : juste compter les types d'événements
        let mut file_mods = 0;
        let mut proc_creates = 0;
        
        for event in log.iter().rev().take(100) { // Analyser les 100 derniers événements
            match event {
                ActivityEvent::FileModification { .. } => file_mods += 1,
                ActivityEvent::ProcessCreation { .. } => proc_creates += 1,
                _ => {}
            }
        }
        
        // Si beaucoup de modifications de fichiers suivies de créations de processus
        if file_mods > 5 && proc_creates > 3 {
            suspicious_patterns.push("file_mod_then_proc_create".to_string());
        }
        
        suspicious_patterns
    }
}
