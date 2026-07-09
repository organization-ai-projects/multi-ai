use std::fs;
use std::path::Path;
use serde::{Serialize, de::DeserializeOwned};
use crate::models::{GlobalMetadata, IaList};

fn load_or_init<T, F>(filename: &str, init: F, project_base: &Path) -> T 
where 
    T: DeserializeOwned + Serialize,
    F: FnOnce(&Path) -> T
{
    let path = project_base.join(filename);
    if let Ok(data) = fs::read(&path) {
        bincode::deserialize(&data).unwrap_or_else(|_| init(project_base))
    } else {
        let data = init(project_base);
        save(&data, &path);
        data
    }
}

fn save<T: Serialize>(data: &T, path: &Path) {
    if let Ok(bytes) = bincode::serialize(data) {
        let _ = fs::write(path, bytes);
    }
}

pub fn load_or_init_global_metadata(project_base: &Path) -> GlobalMetadata {
    load_or_init("metadata.bin", |_| GlobalMetadata::default(), project_base)
}

pub fn save_global_metadata(metadata: &GlobalMetadata, project_base: &Path) {
    save(metadata, &project_base.join("metadata.bin"));
}

pub fn load_or_init_ia_list(project_base: &Path) -> IaList {
    load_or_init("ia_list.bin", crate::setup::generate_initial_metadata, project_base)
}

pub fn save_ia_list(list: &IaList, project_base: &Path) {
    save(list, &project_base.join("ia_list.bin"));
}
