use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct ProjectConfig {
    pub id: usize,
    pub max_memory: usize,
    pub timeout: u64,
}

impl ProjectConfig {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            max_memory: 512 * 1024 * 1024, // 512MB
            timeout: 30,
        }
    }

    pub fn save(&self, project_dir: &str) -> std::io::Result<()> {
        std::fs::write(
            format!("{}/config.ron", project_dir),
            ron::ser::to_string_pretty(self, Default::default()).unwrap(),
        )
    }
}
