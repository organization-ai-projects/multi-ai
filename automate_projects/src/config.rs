use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub paths: Vec<String>,
}

impl Config {
    pub fn load(config_path: PathBuf) -> Result<Self, String> {
        let content = fs::read_to_string(config_path)
            .map_err(|e| format!("Erreur lecture: {}", e))?;
        
        ron::from_str(&content)
            .map_err(|e| format!("Erreur parsing RON: {}", e))
    }
}
