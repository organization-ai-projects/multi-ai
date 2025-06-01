use crate::nosql_structural::{
    collections::Collection,
    storage::StorageManager
};
use std::path::Path;
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init_workspace(name: String) -> Result<(), String> {
    let collection = Collection {
        _id: Uuid::now_v7(),
        name,
        documents: Vec::new(),
        indexes: HashMap::new(),
        references: Vec::new(),
    };

    let storage = StorageManager::new(Path::new(".").to_path_buf());
    storage.save(&collection)?;
    Ok(())
}

pub fn scan_workspace(collection: &Collection) -> Result<(), String> {
    let mut updated_collection = collection.clone();
    
    for path in &collection.scan_paths {
        let found_projects = scanner::scan_projects(Path::new(path), &collection.excluded_paths);
        updated_collection.documents.extend(
            found_projects.into_iter()
                .map(|p| p.document)
        );
    }
    
    // Sauvegarde de la collection mise à jour
    let content = ron::ser::to_string_pretty(&updated_collection, PrettyConfig::default())
        .map_err(|e| format!("Erreur sérialisation : {}", e))?;

    fs::write("workspace.ron", content)
        .map_err(|e| format!("Erreur écriture : {}", e))?;

    let storage = StorageManager::new(Path::new(".").to_path_buf());
    storage.save(&updated_collection)?;
    println!("✅ {} projets trouvés et sauvegardés", updated_collection.documents.len());
    Ok(())
}
