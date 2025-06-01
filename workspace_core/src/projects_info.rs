use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectCategory {
    pub id: Uuid,
    pub name: String,     // "agents", "memory", "tools"
    pub projects: Vec<Project>,  // Projets directement imbriqués
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: Uuid,         // comme le _id MongoDB
    pub name: String,     // identifiant métier (ex: "graph_memory_ai")
    pub status: bool,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    pub workspace_name: String,
    pub scan_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
    pub categories: Vec<ProjectCategory>,
}

// Pour le scanning
pub struct DiscoveredProject {
    pub path: PathBuf,
    pub project: Project,
    pub category_name: String,  // On garde juste le nom de la catégorie pour le scan
}
