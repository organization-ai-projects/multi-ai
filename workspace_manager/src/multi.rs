use std::fs::{self, ReadDir}; // Importer `ReadDir`
use std::path::{Path, PathBuf}; // Ajout des imports nécessaires
use serde::Serialize; // Importer Serialize

/// Représente un sous-projet intelligent détecté
#[derive(Serialize)] // Ajouter Serialize pour permettre la sérialisation
pub struct SubProject {
    pub name: String,
    pub root_path: PathBuf,
    pub container_type: String, // "ide", "intelli", "graphver"
}

/// Scanne récursivement un dossier à la recherche de projets intelligents
pub fn scan_sub_projects(root: &Path) -> Vec<SubProject> {
    let entries: ReadDir = fs::read_dir(root).unwrap_or_else(|_| fs::read_dir(".").unwrap()); // Fournir un dossier par défaut
    entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_dir())
        .map(|entry| SubProject {
            name: entry.file_name().into_string().unwrap_or_default(),
            container_type: "default".to_string(),
            root_path: entry.path(),
        })
        .collect()
}
