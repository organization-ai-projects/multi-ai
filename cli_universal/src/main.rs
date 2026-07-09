use ron::de::from_str;
use serde::Deserialize;
use std::fs;
use std::path::Path;
use workspace_core::nosql_structural::storage::StorageManager;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ProjectType {
    #[serde(rename = "ai_agent")]
    AiAgent {
        id: String,
        status: bool,
        #[serde(default)]
        tags: Option<Vec<String>>,
    },
    #[serde(rename = "tool")]
    Tool {
        id: String,
        status: bool,
    }
}

fn main() {
    let storage = StorageManager::new(Path::new(".").to_path_buf());
    
    // Mode humain = true pour lire le RON
    match storage.load::<ProjectType>("workspace", true) {
        Ok(workspace) => {
            println!("Workspace : {}", workspace.name);
            
            // Affichage des projets détectés
            println!("\nAgents IA disponibles :");
            for path in workspace.scan_paths {
                println!("• Dans {} :", path);
                // TODO: Lecture du résultat du scan depuis un fichier généré par workspace_core
            }
        }
        Err(e) => eprintln!("Erreur lecture workspace : {}", e),
    }
}
