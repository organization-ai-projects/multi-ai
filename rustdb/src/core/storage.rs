use super::{Collection, Document, error::Result};
use serde::{Serialize, de::DeserializeOwned};
use std::path::PathBuf;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::collections::HashMap;

pub struct StorageManager {
    base_path: PathBuf,
    index_path: PathBuf,
}

impl StorageManager {
    pub fn new(base_path: PathBuf) -> Self {
        let index_path = base_path.join("indexes");
        fs::create_dir_all(&index_path).unwrap_or_default();
        Self { base_path, index_path }
    }

    pub fn save<T: Serialize>(&self, collection: &Collection<T>) -> Result<()> {
        let file_path = self.base_path.join(format!("{}.db", collection.name));
        let writer = BufWriter::new(File::create(file_path)?);
        bincode::serialize_into(writer, collection)?;
        
        // Save indexes
        self.save_indexes(collection)?;
        Ok(())
    }

    fn save_indexes<T: Serialize>(&self, collection: &Collection<T>) -> Result<()> {
        for (field, index) in &collection.indexes {
            let index_path = self.index_path.join(format!("{}_{}.idx", collection.name, field));
            let index_data = bincode::serialize(index)
                .map_err(|e| format!("Erreur sérialisation index : {}", e))?;
            fs::write(&index_path, index_data)
                .map_err(|e| format!("Erreur écriture index : {}", e))?;
        }
        Ok(())
    }

    pub fn load_indexes<T>(&self, collection_name: &str) -> Result<HashMap<String, Index>> {
        let mut indexes = HashMap::new();
        let index_pattern = format!("{}*.idx", collection_name);
        
        for entry in fs::read_dir(&self.index_path)? {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            if file_name.starts_with(&collection_name) {
                let index_data = fs::read(entry.path())?;
                let index: Index = bincode::deserialize(&index_data)
                    .map_err(|e| format!("Erreur désérialisation index : {}", e))?;
                indexes.insert(index.field_path.clone(), index);
            }
        }
        
        Ok(indexes)
    }
}