use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use log::{warn, error};
use sysinfo::{System, SystemExt, ProcessExt, Pid};

#[derive(Clone)]
pub struct ResourceLimiter {
    max_memory_mb: u64,
    max_cpu_percent: f32,
}

impl ResourceLimiter {
    pub fn new() -> Self {
        Self {
            max_memory_mb: 4096, // 4GB par défaut
            max_cpu_percent: 90.0, // 90% CPU max
        }
    }
    
    pub fn with_limits(mut self, max_memory_mb: u64, max_cpu_percent: f32) -> Self {
        self.max_memory_mb = max_memory_mb;
        self.max_cpu_percent = max_cpu_percent;
        self
    }
    
    /// Vérifie et applique les limites de ressources, retourne les PID qui dépassent les limites
    pub fn enforce_limits(&self, system: &Arc<Mutex<System>>, process_map: &HashMap<String, u32>) -> Vec<u32> {
        let mut violating_pids = Vec::new();
        let system = system.lock().unwrap();
        
        for (ai_id, pid) in process_map {
            if let Some(process) = system.process(Pid::from(*pid as usize)) {
                // Vérifier la mémoire
                let memory_mb = process.memory() / (1024 * 1024);
                if memory_mb > self.max_memory_mb {
                    error!("L'IA {} (PID {}) dépasse la limite de mémoire: {} MB > {} MB", 
                           ai_id, pid, memory_mb, self.max_memory_mb);
                    violating_pids.push(*pid);
                }
                
                // Vérifier le CPU
                let cpu_percent = process.cpu_usage();
                if cpu_percent > self.max_cpu_percent {
                    warn!("L'IA {} (PID {}) dépasse la limite de CPU: {:.1}% > {:.1}%",
                          ai_id, pid, cpu_percent, self.max_cpu_percent);
                    violating_pids.push(*pid);
                }
            }
        }
        
        violating_pids
    }
    
    pub fn set_memory_limit(&mut self, max_memory_mb: u64) {
        self.max_memory_mb = max_memory_mb;
    }
    
    pub fn set_cpu_limit(&mut self, max_cpu_percent: f32) {
        self.max_cpu_percent = max_cpu_percent;
    }
}
