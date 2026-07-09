use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct IaConfig {
    pub name: String,
    pub manifest_path: String,
    pub args: Vec<String>,
}

pub struct GuardianSettings {
    pub cpu_threshold: f32,
    pub temp_threshold: f32,
    pub check_interval_ms: u64,
    pub cool_down_duration: u64,
}

impl Default for GuardianSettings {
    fn default() -> Self {
        Self {
            cpu_threshold: 75.0,
            temp_threshold: 75.0,
            check_interval_ms: 1000,
            cool_down_duration: 10000,
        }
    }
}

pub fn load_ia_config(path: &str) -> Result<Vec<IaConfig>, String> {
    match fs::read_to_string(path) {
        Ok(config_str) => {
            match ron::de::from_str(&config_str) {
                Ok(config) => Ok(config),
                Err(e) => Err(format!("Erreur parsing RON: {}", e)),
            }
        },
        Err(e) => Err(format!("Impossible de lire {}: {}", path, e)),
    }
}
