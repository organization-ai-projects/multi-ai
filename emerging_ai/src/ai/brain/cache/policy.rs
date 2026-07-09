use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub enum EvictionStrategy {
    LRU,
    FIFO,
    Custom(String),
}

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub enum FlushPolicy {
    Interval(u64),
    OnThreshold(usize),
    OnTransaction,
}

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct CachePolicy {
    pub entity_actions: HashMap<String, Vec<String>>, // entité -> actions permises
    pub max_cache_size: usize,
    pub eviction_strategy: EvictionStrategy,
    pub flush_policy: FlushPolicy,
    pub transaction_mode: bool,
}

impl Default for CachePolicy {
    fn default() -> Self {
        let mut entity_actions = HashMap::new();
        // Actions de base pour chaque type d'entité
        entity_actions.insert(
            "node".to_string(),
            vec!["save".to_string(), "get".to_string(), "list".to_string()],
        );
        entity_actions.insert(
            "link".to_string(),
            vec!["save".to_string(), "get".to_string(), "list".to_string()],
        );
        // etc...

        Self {
            entity_actions,
            max_cache_size: 1000,
            eviction_strategy: EvictionStrategy::LRU,
            flush_policy: FlushPolicy::Interval(300),
            transaction_mode: false,
        }
    }
}

impl CachePolicy {
    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(ron::from_str(&data)?)
    }

    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let serialized = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        std::fs::write(path, serialized)
    }

    pub fn is_action_allowed(&self, entity_type: &str, action: &str) -> bool {
        self.entity_actions
            .get(entity_type)
            .map(|actions| actions.contains(&action.to_string()))
            .unwrap_or(false)
    }
}
