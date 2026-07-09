use crate::paths::{get_component_path, get_project_root};
use crate::context::{get_project_name, is_multi_project};

/// Informations complètes sur le projet courant
pub struct WorkspaceInfo {
    pub name: String,
    pub root_path: std::path::PathBuf,
    pub components: Vec<String>,
}

impl WorkspaceInfo {
    /// Construit une vue complète du projet courant
    pub fn new() -> Self {
        let name = get_project_name();
        let root_path = if is_multi_project() {
            get_project_root(&name)
        } else {
            get_component_path(".")
        };
        let components = std::fs::read_dir(&root_path)
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.path().is_dir())
                    .map(|entry| entry.file_name().to_string_lossy().into_owned())
                    .collect()
            })
            .unwrap_or_else(|_| vec![]);

        WorkspaceInfo {
            name,
            root_path,
            components,
        }
    }
}
