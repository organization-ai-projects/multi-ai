use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct ProjectCache {
    base_dir: PathBuf,
    index: HashMap<String, ProjectState>,
}

#[derive(Default, Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)] // Ajouter Clone
struct ProjectState {
    last_version: String,
    last_scan: u64,
    file_hashes: HashMap<String, String>,
}

impl ProjectCache {
    pub fn new(cache_dir: &Path) -> Self {
        std::fs::create_dir_all(cache_dir).ok();
        Self {
            base_dir: cache_dir.to_owned(),
            index: HashMap::new(),
        }
    }

    pub fn load_project(&mut self, project_id: &str) -> ProjectState {
        let cache_file = self.base_dir.join(format!("{}.cache", project_id));
        if let Ok(content) = std::fs::read(cache_file) {
            bincode_next::decode_from_slice(&content, bincode_next::config::standard()).map(|(v, _)| v).unwrap_or_default()
        } else {
            ProjectState::default()
        }
    }
}
