use std::fs;
use std::path::{Path, PathBuf};
use ron::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WorkspaceConfig {
    pub workspace_name: String,
    pub scan_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ProjectType {
    // ...existing ProjectType code...
}

pub struct DiscoveredProject {
    // ...existing DiscoveredProject code...
}

pub struct Workspace {
    pub config: WorkspaceConfig,
    pub projects: Vec<DiscoveredProject>,
}

impl Workspace {
    pub fn load() -> Result<Self, String> {
        let config = Self::load_config()?;
        let mut projects = Vec::new();
        
        for scan_path in &config.scan_paths {
            Self::scan_projects(Path::new(scan_path), &config.excluded_paths, &mut projects);
        }

        Ok(Self { config, projects })
    }

    fn load_config() -> Result<WorkspaceConfig, String> {
        let content = fs::read_to_string("workspace.ron")
            .map_err(|e| format!("Erreur lecture workspace.ron: {}", e))?;
            
        from_str(&content)
            .map_err(|e| format!("Erreur parsing workspace.ron: {}", e))
    }

    fn scan_projects(path: &Path, excluded: &[String], projects: &mut Vec<DiscoveredProject>) {
        // ...existing scan logic moved here...
    }
}
