use std::fs;
use serde::{Serialize, de::DeserializeOwned};
use crate::models::{GlobalMetadata, IaList};

fn load_or_init<T, F>(filename: &str, init: F) -> T 
where 
    T: DeserializeOwned,
    F: FnOnce() -> T
{
    if let Ok(data) = fs::read(filename) {
        bincode::deserialize(&data).unwrap_or_else(|_| init())
    } else {
        let data = init();
        save(&data, filename);
        data
    }
}

fn save<T: Serialize>(data: &T, filename: &str) {
    if let Ok(bytes) = bincode::serialize(data) {
        let _ = fs::write(filename, bytes);
    }
}

pub fn load_or_init_global_metadata() -> GlobalMetadata {
    load_or_init("metadata.bin", GlobalMetadata::default)
}

pub fn save_global_metadata(metadata: &GlobalMetadata) {
    save(metadata, "metadata.bin")
}

pub fn load_or_init_ia_list() -> IaList {
    load_or_init("ia_list.bin", crate::setup::generate_initial_metadata)
}

pub fn save_ia_list(list: &IaList) {
    save(list, "ia_list.bin")
}
