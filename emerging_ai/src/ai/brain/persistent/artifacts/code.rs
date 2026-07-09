//utilise memory/artifacts/code.rs en y rajoutant la persistance
use crate::brain::memory::artifacts::code::*;
use std::fs;
use std::path::Path;
use bincode;
use ron::ser::{to_string_pretty, PrettyConfig};
use ron::de::from_str;

impl MemoryCode {
    pub fn save_to_bin(&self, path: &Path) -> std::io::Result<()> {
        let serialized = bincode::serialize(self)?;
        fs::write(path, serialized)
    }

    pub fn load_from_bin(path: &Path) -> std::io::Result<Self> {
        let data = fs::read(path)?;
        let deserialized: MemoryCode = bincode::deserialize(&data)?;
        Ok(deserialized)
    }

    pub fn save_to_ron(&self, path: &Path) -> std::io::Result<()> {
        let serialized = to_string_pretty(self, PrettyConfig::default())?;
        fs::write(path, serialized)
    }

    pub fn load_from_ron(path: &Path) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let deserialized: MemoryCode = from_str(&data)?;
        Ok(deserialized)
    }
}