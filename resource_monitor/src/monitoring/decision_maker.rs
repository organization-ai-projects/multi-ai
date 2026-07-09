use std::time::Instant;

use crate::models::ai::{AiInstance, AiState};
use crate::models::resources::ResourceThresholds;

#[derive(Clone)]
pub struct DecisionMaker {
    thresholds: ResourceThresholds,
    last_action: Instant,
}

impl DecisionMaker {
    pub fn new(thresholds: ResourceThresholds) -> Self {
        Self {
            thresholds,
            last_action: Instant::now(),
        }
    }
    
    /// Vérifie si une action doit être prise en fonction des ressources utilisées
    pub fn should_take_action(&self, cpu_usage: f32, mem_usage: f32) -> bool {
        let resources_exceeded = cpu_usage > self.thresholds.max_cpu_usage || 
                                mem_usage > self.thresholds.max_memory_usage;
        
        let cooldown_elapsed = self.last_action.elapsed().as_millis() > 
                               self.thresholds.cooldown_period_ms as u128;
        
        resources_exceeded && cooldown_elapsed
    }
    
    /// Vérifie si des IA en pause peuvent être relancées
    pub fn should_restore(&self, cpu_usage: f32, mem_usage: f32) -> bool {
        let resources_ok = cpu_usage < self.thresholds.max_cpu_usage && 
                          mem_usage < self.thresholds.max_memory_usage;
        
        let cooldown_elapsed = self.last_action.elapsed().as_millis() > 
                               self.thresholds.cooldown_period_ms as u128;
        
        resources_ok && cooldown_elapsed
    }
    
    /// Sélectionne l'IA à mettre en pause (la moins prioritaire)
    pub fn select_ai_to_pause<'a>(&self, ai_instances: &'a [AiInstance]) -> Option<&'a AiInstance> {
        // Trouver les IA actives avec la priorité la plus basse
        let mut active_instances: Vec<&AiInstance> = ai_instances.iter()
            .filter(|ai| matches!(ai.state, AiState::Running))
            .collect();
        
        if active_instances.is_empty() {
            return None;
        }
        
        // Trier par priorité croissante (les moins prioritaires d'abord)
        active_instances.sort_by_key(|ai| ai.priority);
        active_instances.first().copied()
    }
    
    /// Sélectionne l'IA à relancer (la plus prioritaire)
    pub fn select_ai_to_resume<'a>(&self, ai_instances: &'a [AiInstance]) -> Option<&'a AiInstance> {
        // Trouver les IA en pause avec la priorité la plus élevée
        let mut paused_instances: Vec<&AiInstance> = ai_instances.iter()
            .filter(|ai| matches!(ai.state, AiState::Paused))
            .collect();
        
        if paused_instances.is_empty() {
            return None;
        }
        
        // Trier par priorité décroissante (les plus prioritaires d'abord)
        paused_instances.sort_by_key(|ai| std::cmp::Reverse(ai.priority));
        paused_instances.first().copied()
    }
    
    /// Signale qu'une action a été prise (pour respecter le cooldown)
    pub fn action_taken(&mut self) {
        self.last_action = Instant::now();
    }
}