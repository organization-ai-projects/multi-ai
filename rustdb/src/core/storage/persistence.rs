use super::file_system::FileStorage;
use crate::core::error::Result;
use serde::{Serialize, de::DeserializeOwned};

pub trait PersistenceStrategy {
    fn save<T: Serialize>(&self, key: &str, data: &T) -> Result<()>;
    fn load<T: DeserializeOwned>(&self, key: &str) -> Result<T>;
    fn delete(&self, key: &str) -> Result<()>;
    fn exists(&self, key: &str) -> bool;
}

pub struct BinaryPersistence {
    storage: FileStorage,
}

impl BinaryPersistence {
    pub fn new(storage: FileStorage) -> Self {
        Self { storage }
    }
}

impl PersistenceStrategy for BinaryPersistence {
    fn save<T: Serialize>(&self, key: &str, data: &T) -> Result<()> {
        let serialized = bincode_next::encode_to_vec(data, bincode_next::config::standard())?;
        let pages = self.split_into_pages(&serialized);
        
        for (i, page) in pages.iter().enumerate() {
            self.storage.write_page(&format!("{}.bin", key), i as u32, page)?;
        }
        Ok(())
    }

    fn load<T: DeserializeOwned>(&self, key: &str) -> Result<T> {
        let mut data = Vec::new();
        let mut page_id = 0;

        loop {
            match self.storage.read_page(&format!("{}.bin", key), page_id) {
                Ok(page) => data.extend_from_slice(&page),
                Err(_) => break,
            }
            page_id += 1;
        }

        Ok(bincode_next::decode_from_slice(&data, bincode_next::config::standard()).map(|(v, _)| v)?)
    }

    fn delete(&self, key: &str) -> Result<()> {
        self.storage.delete_file(&format!("{}.bin", key))
    }

    fn exists(&self, key: &str) -> bool {
        self.storage.file_exists(&format!("{}.bin", key))
    }
}