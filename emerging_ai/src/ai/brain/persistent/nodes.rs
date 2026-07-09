use crate::brain::memory::nodes::*;
use std::fs;
use std::path::Path;
use bincode;
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;

//contient les noeuds du graphe mémoire de l'IA avec persistence
//utilise ce qui a déjà été défini dans memory/nodes.rs

impl MemoryNode {
    pub fn save_to_bin(&self, path: &Path) -> std::io::Result<()> {
        let serialized = bincode::serialize(self)?;
        fs::write(path, serialized)
    }

    pub fn load_from_bin(path: &Path) -> std::io::Result<Self> {
        let data = fs::read(path)?;
        let deserialized: MemoryNode = bincode::deserialize(&data)?;
        Ok(deserialized)
    }

    pub fn save_to_ron(&self, path: &Path) -> std::io::Result<()> {
        let serialized = to_string_pretty(self, PrettyConfig::default())?;
        fs::write(path, serialized)
    }

    pub fn load_from_ron(path: &Path) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let deserialized: MemoryNode = from_str(&data)?;
        Ok(deserialized)
    }
}