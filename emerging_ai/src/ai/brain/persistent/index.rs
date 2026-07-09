use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GlobalIndex {
    pub nodes: HashMap<Uuid, u64>, // UUID -> Timestamp
    pub links: HashMap<Uuid, u64>, // UUID -> Timestamp
    pub logs: HashMap<Uuid, u64>,  // UUID -> Timestamp
}

impl GlobalIndex {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            logs: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, uuid: Uuid, timestamp: u64) {
        self.nodes.insert(uuid, timestamp);
    }

    pub fn add_link(&mut self, uuid: Uuid, timestamp: u64) {
        self.links.insert(uuid, timestamp);
    }

    pub fn add_log(&mut self, uuid: Uuid, timestamp: u64) {
        self.logs.insert(uuid, timestamp);
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let serialized = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, serialized)
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let deserialized: GlobalIndex = ron::de::from_str(&data)?;
        Ok(deserialized)
    }
}
