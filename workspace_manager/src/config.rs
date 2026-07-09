// config.rs - Gestion de la configuration du projet
use std::fs;
use std::path::PathBuf; // Supprimer `Path` car il n'est pas utilisé
use serde::{Deserialize, Serialize}; // Ajout des imports manquants

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectConfig {
    pub name: String,
    pub mode: Option<String>,
    pub multi_project: Option<bool>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        ProjectConfig {
            name: "default_project".into(),
            mode: None,
            multi_project: None,
        }
    }
}

// Retourne le chemin du fichier de configuration du projet
pub fn get_config_path() -> PathBuf { // Rendre cette fonction publique
    let mut path = PathBuf::from(".");
    path.push("project_config.ron");
    path
}

// Sauvegarde la configuration du projet dans le fichier
pub fn save_config(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();
    let serialized = ron::ser::to_string(config)?;
    fs::write(path, serialized)?;
    Ok(())
}

// Charge la configuration du projet depuis le fichier
pub fn load_config() -> Result<ProjectConfig, Box<dyn std::error::Error>> {
    let path = get_config_path();
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let config: ProjectConfig = ron::de::from_str(&content)?;
        Ok(config)
    } else {
        Err("Configuration file not found".into())
    }
}
