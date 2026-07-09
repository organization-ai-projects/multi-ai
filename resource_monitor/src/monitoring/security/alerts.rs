use log::warn;
use crate::models::security::{SecurityLevel, SecurityAlert};

#[derive(Clone)]
pub struct AlertManager {
    security_level: SecurityLevel,
    alert_history: Vec<SecurityAlert>,
}

impl AlertManager {
    pub fn new(security_level: SecurityLevel) -> Self {
        Self {
            security_level,
            alert_history: Vec::new(),
        }
    }
    
    pub fn set_security_level(&mut self, level: SecurityLevel) {
        self.security_level = level;
    }
    
    pub fn get_security_level(&self) -> SecurityLevel {
        self.security_level.clone()
    }
    
    pub fn record_alert(&mut self, alert: SecurityAlert) {
        // Enregistrer l'alerte
        self.alert_history.push(alert.clone());
        
        // Selon le niveau de sécurité, prendre des actions différentes
        match self.security_level {
            SecurityLevel::Standard => {
                // Juste enregistrer l'alerte
                warn!("Alerte de sécurité (niveau standard): {:?}", alert);
            },
            SecurityLevel::High => {
                // Alerte plus visible
                warn!("⚠️ ALERTE DE SÉCURITÉ (niveau élevé): {:?}", alert);
            },
            SecurityLevel::Critical => {
                // Alerte critique
                warn!("🔴 ALERTE CRITIQUE DE SÉCURITÉ: {:?}", alert);
            },
        }
    }
    
    pub fn get_recent_alerts(&self, count: usize) -> Vec<&SecurityAlert> {
        self.alert_history.iter().rev().take(count).collect()
    }
    
    pub fn clear_history(&mut self) {
        self.alert_history.clear();
    }
}
