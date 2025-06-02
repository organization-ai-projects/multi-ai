use std::fs;
use crate::models::{GlobalMetadata, IaList};
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

pub fn load_or_init_ia_list() -> IaList {
    if let Ok(data) = fs::read("ia_list.bin") {
        bincode::deserialize(&data).unwrap_or_else(|_| crate::setup::generate_initial_metadata())
    } else {
        let list = crate::setup::generate_initial_metadata();
        save_ia_list(&list);
        list
    }
}

pub fn save_ia_list(list: &IaList) {
    if let Ok(data) = bincode::serialize(list) {
        let _ = fs::write("ia_list.bin", data);
    }
}
