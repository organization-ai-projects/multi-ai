use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use sysinfo::System;

use crate::models::resources::ResourceThresholds;
use crate::error::ResourceMonitorError;
use super::watcher::ResourceWatcher;
use super::controller::ProcessController;
use super::tracker::ProcessTracker;
use super::limits::ResourceLimiter;

#[derive(Clone)]
pub struct ResourceManager {
    system: Arc<Mutex<System>>,
    thresholds: ResourceThresholds,
    watcher: ResourceWatcher,
    controller: ProcessController,
    tracker: ProcessTracker,
    limiter: ResourceLimiter,
    process_map: HashMap<String, u32>, // AI ID -> PID
}

impl ResourceManager {
    pub fn new(system: Arc<Mutex<System>>, thresholds: ResourceThresholds) -> Self {
        Self {
            system: system.clone(),
            thresholds,
            watcher: ResourceWatcher::new(system.clone()),
            controller: ProcessController::new(),
            tracker: ProcessTracker::new(3), // Limite par défaut
            limiter: ResourceLimiter::new(),
            process_map: HashMap::new(),
        }
    }
    
    pub fn get_system_stats(&self) -> Result<(f32, f32), ResourceMonitorError> {
        self.watcher.get_system_stats()
    }
    
    pub fn get_process_stats(&self, pid: u32) -> Result<(f32, u64), ResourceMonitorError> {
        self.watcher.get_process_stats(pid)
    }
    
    pub fn register_process(&mut self, pid: u32) {
        self.tracker.register_parent_process(pid);
    }
    
    pub fn pause_process(&self, pid: u32) -> Result<(), ResourceMonitorError> {
        self.controller.pause_process(pid)
    }
    
    pub fn resume_process(&self, pid: u32) -> Result<(), ResourceMonitorError> {
        self.controller.resume_process(pid)
    }
    
    pub fn terminate_process(&self, pid: u32) -> Result<(), ResourceMonitorError> {
        self.controller.terminate_process(pid)
    }
    
    pub fn get_process_map(&self) -> &HashMap<String, u32> {
        &self.process_map
    }
    
    pub fn add_process_mapping(&mut self, ai_id: &str, pid: u32) {
        self.process_map.insert(ai_id.to_string(), pid);
    }
    
    pub fn check_for_new_processes(&mut self) -> Vec<(u32, u32)> {
        let sys = self.system.lock().unwrap();
        self.tracker.check_for_new_child_processes(&sys)
    }
    
    pub fn set_max_processes_per_ai(&mut self, limit: usize) {
        self.tracker.set_max_processes_per_ai(limit);
    }
    
    pub fn enforce_resource_limits(&self) -> Vec<u32> {
        self.limiter.enforce_limits(&self.system, &self.process_map)
    }
}
