use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::path::Path;
use std::time::{SystemTime, Duration};
use std::fs;
use sysinfo::{System, SystemExt, CpuExt, ProcessExt};
use log::{debug, error, warn};

use crate::error::ResourceMonitorError;

#[derive(Clone)]
pub struct ResourceWatcher {
    system: Arc<Mutex<System>>,
    // Surveillance de fichiers
    file_timestamps: Arc<Mutex<HashMap<String, SystemTime>>>,

}

impl ResourceWatcher {
    pub fn new(system: Arc<Mutex<System>>) -> Self {
        Self { system }
    }
    
    pub fn get_system_stats(&self) -> Result<(f32, f32), ResourceMonitorError> {
        let mut system = self.system.lock().unwrap();
        system.refresh_all();
        
        // CPU usage
        let cpu_usage = system.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / 
                         system.cpus().len() as f32;
        
        // Memory usage
        let total_mem = system.total_memory() as f32;
        let used_mem = (system.total_memory() - system.available_memory()) as f32;
        let mem_usage = (used_mem / total_mem) * 100.0;
        
        debug!("Stats système - CPU: {:.1}%, Mémoire: {:.1}%", cpu_usage, mem_usage);
        Ok((cpu_usage, mem_usage))
    }
    
    pub fn get_process_stats(&self, pid: u32) -> Result<(f32, u64), ResourceMonitorError> {
        let system = self.system.lock().unwrap();
        
        if let Some(process) = system.process(sysinfo::Pid::from(pid as usize)) {
            let cpu_usage = process.cpu_usage();
            let memory = process.memory();
            debug!("Process {} - CPU: {:.1}%, Mémoire: {:.1}MB", 
                  pid, cpu_usage, memory / (1024 * 1024));
            return Ok((cpu_usage, memory));
        }
        
        Err(ResourceMonitorError::MonitoringError(format!("Processus {} non trouvé", pid)))
    }
    
    pub fn refresh_system(&self) {
        let mut system = self.system.lock().unwrap();
        system.refresh_all();
    }
    
    /// Vérifie si des fichiers dans un répertoire ont été modifiés depuis la dernière vérification
    pub fn check_directory_changes(&self, dir_path: &str) -> Result<Vec<String>, ResourceMonitorError> {
        let path = Path::new(dir_path);
        if !path.exists() || !path.is_dir() {
            return Err(ResourceMonitorError::MonitoringError(
                format!("Le répertoire {} n'existe pas", dir_path)
            ));
        }
        
        let mut modified_files = Vec::new();
        let mut timestamps = self.file_timestamps.lock().unwrap();
        
        // Parcourir tous les fichiers dans le répertoire
        self.scan_directory_recursive(path, &mut modified_files, &mut timestamps)?;
        
        Ok(modified_files)
    }
    
    /// Scan récursif des fichiers
    fn scan_directory_recursive(&self, 
                              dir_path: &Path, 
                              modified_files: &mut Vec<String>,
                              timestamps: &mut HashMap<String, SystemTime>) -> Result<(), ResourceMonitorError> {
        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Récursion pour les sous-répertoires
                        self.scan_directory_recursive(&path, modified_files, timestamps)?;
                    } else {
                        // Vérifier si le fichier a été modifié
                        if let Ok(metadata) = fs::metadata(&path) {
                            if let Ok(modified) = metadata.modified() {
                                let path_str = path.to_string_lossy().to_string();
                                
                                if let Some(last_modified) = timestamps.get(&path_str) {
                                    // Le fichier était déjà connu, vérifier s'il a été modifié
                                    if modified > *last_modified {
                                        modified_files.push(path_str.clone());
                                        timestamps.insert(path_str, modified);
                                    }
                                } else {
                                    // Nouveau fichier
                                    timestamps.insert(path_str, modified);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Vérifie si un fichier ou répertoire spécifique a été modifié récemment
    pub fn has_recent_modifications(&self, path: &str) -> Result<bool, ResourceMonitorError> {
        let path_obj = Path::new(path);
        if !path_obj.exists() {
            return Err(ResourceMonitorError::MonitoringError(
                format!("Le chemin {} n'existe pas", path)
            ));
        }
        
        if let Ok(metadata) = fs::metadata(path_obj) {
            if let Ok(modified) = metadata.modified() {
                // Vérifier si la modification est récente (moins de 5 minutes)
                if let Ok(duration) = SystemTime::now().duration_since(modified) {
                    return Ok(duration < Duration::from_secs(300));
                }
            }
        }
        
        Ok(false)
    }
    
    /// Vérifie si l'utilisation CPU du système dépasse un seuil (en %)
    pub fn is_system_cpu_overload(&self, threshold: f32) -> bool {
        if let Ok((cpu_usage, _)) = self.get_system_stats() {
            cpu_usage > threshold
        } else {
            false
        }
    }

    /// Vérifie si l'utilisation CPU d'un processus dépasse un seuil (en %)
    pub fn is_process_cpu_overload(&self, pid: u32, threshold: f32) -> bool {
        if let Ok((cpu_usage, _)) = self.get_process_stats(pid) {
            cpu_usage > threshold
        } else {
            false
        }
    }
}