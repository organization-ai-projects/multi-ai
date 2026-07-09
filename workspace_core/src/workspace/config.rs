use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub struct WorkspaceConfig {
    pub name: String,
    pub scan_paths: Vec<String>,
    pub excluded_paths: Vec<String>,
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            name: "workspace".to_string(),
            scan_paths: vec!["./".to_string()],
            excluded_paths: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
                ".vscode".to_string(),
            ],
        }
    }
}
