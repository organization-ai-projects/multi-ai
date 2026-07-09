use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use log::info;
use sysinfo::System;
use std::path::PathBuf;

use crate::models::ai::AiInstance;
use crate::models::resources::ResourceThresholds;
use crate::error::ResourceMonitorError;
use crate::monitoring::resource::ResourceManager;
use crate::monitoring::security::SecurityManager;

/// Orchestrateur principal pour la surveillance et le contrôle des IA
pub struct ResourceMonitor {
    system: Arc<Mutex<System>>,
    thresholds: ResourceThresholds,
    ai_instances: Vec<AiInstance>,
    is_running: Arc<Mutex<bool>>,
    resource_manager: ResourceManager,
    security_manager: SecurityManager,
}

impl ResourceMonitor {
    pub fn new(thresholds: Option<ResourceThresholds>) -> Self {
        let system = Arc::new(Mutex::new(System::new_all()));
        let thresholds = thresholds.unwrap_or_default();
        let resource_manager = ResourceManager::new(system.clone(), thresholds.clone());
        let security_manager = SecurityManager::new(system.clone());

        Self {
            system,
            thresholds,
            ai_instances: Vec::new(),
            is_running: Arc::new(Mutex::new(false)),
            resource_manager,
            security_manager,
        }
    }
    
    // Gestion des IA
    pub fn register_ai_instance(&mut self, id: String, priority: u8) -> Result<(), ResourceMonitorError> {
        if self.ai_instances.iter().any(|ai| ai.id == id) {
            return Err(ResourceMonitorError::ConfigurationError(format!("L'IA avec l'ID {} est déjà enregistrée", id)));
        }
        
        self.ai_instances.push(AiInstance {
            id,
            state: AiState::Running,
            priority,
        });
        
        Ok(())
    }
    
    pub fn register_ai_with_sandbox(&mut self, id: String, priority: u8) -> Result<(), ResourceMonitorError> {
        self.register_ai_instance(id.clone(), priority)?;
        self.security_manager.register_ai_in_zones(&id)?;
        info!("IA {} enregistrée avec sandbox sécurisé", id);
        Ok(())
    }
    
    pub fn associate_ai_with_process(&mut self, ai_id: &str, process_id: u32) -> Result<(), ResourceMonitorError> {
        if !self.ai_instances.iter().any(|ai| ai.id == ai_id) {
            return Err(ResourceMonitorError::ConfigurationError(
                format!("IA avec ID {} non enregistrée", ai_id)
            ));
        }
        
        self.process_map.insert(ai_id.to_string(), process_id);
        info!("IA {} associée au processus {}", ai_id, process_id);
        self.resource_manager.register_process(process_id);
        Ok(())
    }
    
    // Contrôle des processus
    pub fn pause_ai(&mut self, id: &str) -> Result<(), ResourceMonitorError> {
        // Trouver l'IA et vérifier son état
        let ai = self.ai_instances.iter_mut()
            .find(|ai| ai.id == id)
            .ok_or_else(|| ResourceMonitorError::ConfigurationError(format!("IA avec ID {} non trouvée", id)))?;
        
        // Déléguer le contrôle au ProcessController
        if let Some(pid) = self.process_map.get(id) {
            self.controller.pause_process(*pid)?;
        }

        // Mettre à jour l'état
        ai.update_state_to_paused()?;
        info!("IA {} mise en pause", id);
        Ok(())
    }
    
    pub fn resume_ai(&mut self, id: &str) -> Result<(), ResourceMonitorError> {
        // Délégation similaire au controller
        let ai = self.ai_instances.iter_mut()
            .find(|ai| ai.id == id)
            .ok_or_else(|| ResourceMonitorError::ConfigurationError(format!("IA avec ID {} non trouvée", id)))?;
        
        if let Some(pid) = self.process_map.get(id) {
            self.controller.resume_process(*pid)?;
        }

        ai.update_state_to_running()?;
        info!("IA {} reprise", id);
        Ok(())
    }
    
    // Surveillance
    pub async fn start_monitoring(&self) -> Result<(), ResourceMonitorError> {
        // Délègue au module de surveillance
        crate::monitoring::monitoring_loop::start_monitoring_loop(
            self.system.clone(),
            self.thresholds.clone(),
            self.is_running.clone(),
            self.clone(),
            self.security_manager.clone(),
            self.process_tracker.clone(),
        ).await
    }
    
    pub fn stop_monitoring(&self) -> Result<(), ResourceMonitorError> {
        let mut is_running = self.is_running.lock().unwrap();
        *is_running = false;
        info!("Surveillance des ressources arrêtée");
        Ok(())
    }
    
    // Accès aux statistiques
    pub fn get_system_stats(&self) -> Result<(f32, f32), ResourceMonitorError> {
        self.resource_manager.get_system_stats()
    }
    
    pub fn get_process_stats(&self, pid: u32) -> Result<(f32, u64), ResourceMonitorError> {
        self.resource_manager.get_process_stats(pid)
    }
    
    // Gestion de la sécurité
    pub fn set_security_level(&mut self, level: crate::models::security::SecurityLevel) {
        self.security_manager.set_security_level(level);
    }
    
    pub fn add_monitored_directory(&mut self, path: &str) -> Result<(), ResourceMonitorError> {
        self.security_manager.add_monitored_directory(path)
    }
    
    pub fn add_forbidden_path(&mut self, path: &str) {
        self.security_manager.add_forbidden_path(path);
    }
    
    // Méthodes d'accès pour l'interface utilisateur
    pub fn get_ai_instances(&self) -> Vec<AiInstance> {
        self.ai_instances.clone()
    }
    
    pub fn get_process_map(&self) -> &HashMap<String, u32> {
        &self.resource_manager.get_process_map()
    }
    
    // Méthode helper pour le clonage
    pub fn clone(&self) -> Self {
        Self {
            system: self.system.clone(),
            thresholds: self.thresholds.clone(),
            ai_instances: self.ai_instances.clone(),
            is_running: self.is_running.clone(),
            resource_manager: self.resource_manager.clone(),
            security_manager: self.security_manager.clone(),
        }
    }
}