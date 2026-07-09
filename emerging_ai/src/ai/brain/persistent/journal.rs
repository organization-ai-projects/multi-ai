use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JournalEntry {
    pub action: String, // "update", "delete", etc.
    pub entity_type: String, // "node", "link", etc.
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Journal {
    pub entries: Vec<JournalEntry>,
}

impl Journal {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, action: &str, entity_type: &str, uuid: &str) {
        self.entries.push(JournalEntry {
            action: action.to_string(),
            entity_type: entity_type.to_string(),
            uuid: uuid.to_string(),
        });
    }

    pub fn save_to_file(&self, path: &Path) -> std::io::Result<()> {
        let serialized = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())?;
        fs::write(path, serialized)
    }

    pub fn load_from_file(path: &Path) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let deserialized: Journal = ron::de::from_str(&data)?;
        Ok(deserialized)
    }

    /// Marque une opération comme terminée
    pub fn mark_completed(&mut self, uuid: &str) {
        self.entries.retain(|entry| entry.uuid != uuid);
    }

    /// Restaure les opérations incomplètes
    pub fn restore_incomplete_operations(&self) -> Vec<JournalEntry> {
        self.entries.clone()
    }
}
