use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct MonitoringLog {
    pub timestamp: u64,
    pub ia_name: String,
    pub cpu_usage: f32,
    pub temperature: f32,
    pub action_taken: String,
    pub details: String,
}

#[derive(Clone)]
pub struct Logger {
    bin_file: Arc<Mutex<File>>,
    ron_file: Arc<Mutex<File>>,
    path_base: String,
}

impl Logger {
    pub fn new(base_path: &str) -> Self {
        // Chemins pour les différents formats
        let bin_path = format!("{}.bin", base_path);
        let ron_path = format!("{}.ron", base_path);
        
        let bin_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&bin_path)
            .expect("Impossible d'ouvrir le fichier de log binaire");
            
        let ron_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&ron_path)
            .expect("Impossible d'ouvrir le fichier de log RON");
        
        Self {
            bin_file: Arc::new(Mutex::new(bin_file)),
            ron_file: Arc::new(Mutex::new(ron_file)),
            path_base: base_path.to_string(),
        }
    }
    
    pub fn log(&self, ia_name: &str, cpu: f32, temp: f32, action: &str, details: &str) {
        let log = MonitoringLog {
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            ia_name: ia_name.to_string(),
            cpu_usage: cpu,
            temperature: temp,
            action_taken: action.to_string(),
            details: details.to_string(),
        };
        
        // Log au format binaire (pour traitement par l'IA)
        if let Ok(mut file) = self.bin_file.lock() {
            if let Ok(bin_data) = bincode_next::encode_to_vec(&log, bincode_next::config::standard()) {
                let len = bin_data.len() as u32;
                let _ = file.write_all(&len.to_le_bytes()); // Écrire la taille d'abord
                let _ = file.write_all(&bin_data);          // Puis les données
            }
        }
        
        // Log au format RON (pour lecture humaine)
        if let Ok(serialized) = ron::ser::to_string_pretty(&log, ron::ser::PrettyConfig::default()) {
            if let Ok(mut file) = self.ron_file.lock() {
                let _ = writeln!(file, "{}", serialized);
            }
        }
    }
    
    pub fn file_paths(&self) -> (String, String) {
        (
            format!("{}.bin", self.path_base),
            format!("{}.ron", self.path_base),
        )
    }
}

pub fn create_session_logger(log_dir: &str) -> Logger {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let log_base = format!("{}/guardian_log_{}", log_dir, timestamp);
    Logger::new(&log_base)
}
