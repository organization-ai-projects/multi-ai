use crate::nosql_structural::{collections::Collection, storage::StorageManager};
use crate::scanner;
use crate::schema::ProjectDocument;
use ron::ser::PrettyConfig;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn init_workspace(name: String) -> Result<(), String> {
    let mut collection = ProjectsCollection::new_workspace();
    collection.name = name;
    
    let storage = StorageManager::new(Path::new(".").to_path_buf());
    storage.save(&collection)?;
    Ok(())
}

pub fn scan_workspace(collection: &Collection<ProjectDocument>) -> Result<(), String> {
    let mut updated_collection = collection.clone();
    
    let found_projects: Vec<_> = collection.scan_paths.iter()
        .flat_map(|path| scanner::scan_projects(Path::new(path), &collection.excluded_paths))
        .collect();

    updated_collection.documents = found_projects.into_iter()
        .map(|p| p.document)
        .collect();

    let storage = StorageManager::new(Path::new(".").to_path_buf());
    storage.save(&updated_collection)?;
    
    println!("✅ {} projets trouvés et sauvegardés", updated_collection.documents.len());
    Ok(())
}
