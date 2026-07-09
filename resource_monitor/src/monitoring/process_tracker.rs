use std::collections::{HashMap, HashSet};
use log::{warn, error, info};
use sysinfo::{Pid, Process, ProcessExt, System, SystemExt};

use crate::error::ResourceMonitorError;

#[derive(Clone)]
pub struct ProcessTracker {
    parent_processes: HashSet<u32>,
    child_processes: HashMap<u32, u32>, // Map child PID -> parent PID
    max_processes_per_ai: usize,
}

impl ProcessTracker {
    pub fn new(max_processes_per_ai: usize) -> Self {
        Self {
            parent_processes: HashSet::new(),
            child_processes: HashMap::new(),
            max_processes_per_ai,
        }
    }
    
    pub fn register_parent_process(&mut self, pid: u32) {
        self.parent_processes.insert(pid);
        info!("Processus parent enregistré: {}", pid);
    }
    
    pub fn unregister_parent_process(&mut self, pid: u32) {
        self.parent_processes.remove(&pid);
        
        // Supprimer également tous les enfants associés
        self.child_processes.retain(|_, &mut parent| parent != pid);
        info!("Processus parent désenregistré: {}", pid);
    }
    
    pub fn register_child_process(&mut self, child_pid: u32, parent_pid: u32) -> bool {
        // Vérifier si on ne dépasse pas la limite par parent
        let parent_child_count = self.child_processes.values()
            .filter(|&&p| p == parent_pid)
            .count();
        
        if parent_child_count >= self.max_processes_per_ai {
            warn!("Limite de processus dépassée pour le parent {}. Refus d'enregistrer l'enfant {}.",
                  parent_pid, child_pid);
            return false;
        }
        
        self.child_processes.insert(child_pid, parent_pid);
        info!("Processus enfant enregistré: {} (parent: {})", child_pid, parent_pid);
        true
    }
    
    pub fn unregister_child_process(&mut self, child_pid: u32) {
        self.child_processes.remove(&child_pid);
    }
    
    pub fn set_max_processes_per_ai(&mut self, limit: usize) {
        self.max_processes_per_ai = limit;
        info!("Limite de processus par IA modifiée: {}", limit);
    }
    
    /// Vérifie les nouveaux processus enfants des IA surveillées
    pub fn check_for_new_child_processes(&mut self, system: &System) -> Vec<(u32, u32)> {
        let mut new_children = Vec::new();
        
        // Convertir les PID parents en PIDs sysinfo
        let parent_pids: HashSet<_> = self.parent_processes
            .iter()
            .map(|&pid| Pid::from(pid as usize))
            .collect();
        
        // Rechercher les processus enfants
        for (pid, process) in system.processes() {
            let child_pid = pid.as_u32();
            
            // Si déjà connu, ignorer
            if self.child_processes.contains_key(&child_pid) {
                continue;
            }
            
            // Vérifier si c'est un enfant d'un de nos processus parents
            if let Some(parent) = process.parent() {
                if parent_pids.contains(&parent) {
                    let parent_pid = parent.as_u32();
                    warn!("Nouveau processus enfant détecté: {} (parent: {})", child_pid, parent_pid);
                    new_children.push((child_pid, parent_pid));
                }
            }
        }
        
        new_children
    }
    
    /// Calcule le nombre d'enfants pour un processus parent
    pub fn count_children_for_parent(&self, parent_pid: u32) -> usize {
        self.child_processes.values()
            .filter(|&&p| p == parent_pid)
            .count()
    }
    
    /// Vérifie si un processus enfant appartient à un parent surveillé
    pub fn is_monitored_child(&self, pid: u32) -> bool {
        self.child_processes.contains_key(&pid)
    }
    
    /// Obtient le parent d'un processus enfant
    pub fn get_parent_of(&self, child_pid: u32) -> Option<u32> {
        self.child_processes.get(&child_pid).copied()
    }
}
