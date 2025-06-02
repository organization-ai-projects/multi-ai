use std::fs;
use crate::models::GlobalMetadata;
use bincode;

pub fn load_or_init_global_metadata() -> GlobalMetadata {
    if let Ok(data) = fs::read("metadata.bin") {
        bincode::deserialize(&data).unwrap_or_default()
    } else {
        let metadata = GlobalMetadata::default();
        save_global_metadata(&metadata);
        metadata
    }
}

pub fn save_global_metadata(metadata: &GlobalMetadata) {
    if let Ok(data) = bincode::serialize(metadata) {
        let _ = fs::write("metadata.bin", data);
    }
}
