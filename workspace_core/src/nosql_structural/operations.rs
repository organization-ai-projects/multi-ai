use super::{collections::Collection, storage::StorageManager, document::Document};
use crate::schema::projects::{ProjectDocument, COLLECTION_NAME};
use crate::nosql_structural::search::DocumentMatcher;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

pub trait CollectionOperations<T> {
    fn find(storage: &StorageManager, filter: HashMap<String, String>) -> Vec<T>;
    fn find_one(storage: &StorageManager, filter: HashMap<String, String>) -> Option<T>;
    fn insert_one(storage: &StorageManager, doc: T) -> Result<Uuid, String>;
    fn update_one(storage: &StorageManager, filter: HashMap<String, String>, doc: T) -> Result<(), String>;
    fn delete_one(storage: &StorageManager, filter: HashMap<String, String>) -> Result<(), String>;
}

impl CollectionOperations<ProjectDocument> for Collection<ProjectDocument> {
    fn find(storage: &StorageManager, filter: HashMap<String, String>) -> Vec<ProjectDocument> {
        if let Ok(collection) = storage.load::<ProjectDocument>(COLLECTION_NAME, false) {
            collection.documents.into_iter()
                .filter(|doc| filter.iter().all(|(k, v)| doc.data.matches(k, v)))
                .map(|doc| doc.data)
                .collect()
        } else {
            vec![]
        }
    }

    fn find_one(storage: &StorageManager, filter: HashMap<String, String>) -> Option<ProjectDocument> {
        Self::find(storage, filter).into_iter().next()
    }

    fn insert_one(storage: &StorageManager, doc: ProjectDocument) -> Result<Uuid, String> {
        let mut collection = storage.load::<ProjectDocument>(COLLECTION_NAME, false)
            .unwrap_or_else(|_| Collection::new(COLLECTION_NAME));
        
        let document = Document::from(doc);
        let id = document._id;
        collection.documents.push(document);
        storage.save(&collection)?;
        
        Ok(id)
    }

    fn update_one(storage: &StorageManager, filter: HashMap<String, String>, doc: ProjectDocument) -> Result<(), String> {
        let mut collection = storage.load::<ProjectDocument>(COLLECTION_NAME, false)?;
        
        if let Some(idx) = collection.documents.iter().position(|d| {
            filter.iter().all(|(k, v)| d.data.matches(k, v))
        }) {
            collection.documents[idx].data = doc;
            storage.save(&collection)?;
        }
        
        Ok(())
    }

    fn delete_one(storage: &StorageManager, filter: HashMap<String, String>) -> Result<(), String> {
        let mut collection = storage.load::<ProjectDocument>(COLLECTION_NAME, false)?;
        
        if let Some(idx) = collection.documents.iter().position(|d| {
            filter.iter().all(|(k, v)| d.data.matches(k, v))
        }) {
            collection.documents.remove(idx);
            storage.save(&collection)?;
        }
        
        Ok(())
    }
}
