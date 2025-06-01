use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use std::collections::HashMap;
use super::references::DbRef;
use super::error::{DatabaseError, Result};
use super::cache::QueryCache;
use std::num::NonZeroUsize;
use super::document::Document;

// Structure de base d'une collection dans notre système
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Collection<T> {
    pub _id: Uuid,
    pub name: String,
    pub documents: Vec<Document<T>>,
    pub indexes: HashMap<String, Index>,
    cached_fields: HashMap<String, QueryCache<String, Vec<Uuid>>>, // Ajout du cache
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Index {
    pub field: String,
    pub unique: bool,
    pub sparse: bool,
    pub cached: bool,
}

// Uniquement les méthodes essentielles pour la gestion des documents
impl<T: serde::Serialize> Collection<T> {
    pub fn new(name: &str) -> Self {
        Self {
            _id: Uuid::now_v7(),
            name: name.to_string(),
            documents: Vec::new(),
            indexes: HashMap::new(),
            cached_fields: HashMap::new(), // Initialisation du cache
        }
    }

    pub fn add_index(&mut self, field: &str, unique: bool) {
        self.indexes.insert(field.to_string(), Index { 
            field: field.to_string(), 
            unique, 
            sparse: false,
            cached: false,
        });
    }

    pub fn ensure_index(&mut self, field: &str, options: IndexOptions) -> Result<()> {
        if !self.indexes.contains_key(field) {
            self.indexes.insert(field.to_string(), Index {
                field: field.to_string(),
                unique: options.unique,
                sparse: options.sparse,
                cached: options.cached,
            });
            
            if options.cached {
                self.build_cache_for_field(field)?;
            }
        }
        Ok(())
    }

    fn build_cache_for_field(&mut self, field: &str) -> Result<()> {
        let cache = QueryCache::new(NonZeroUsize::new(1000).unwrap());
        
        for doc in &self.documents {
            if let Some(value) = doc.get_field_value(field) {
                let existing = cache.get(&value).unwrap_or_default();
                let mut ids = existing;
                ids.push(doc._id);
                cache.insert(value, ids);
            }
        }

        self.cached_fields.insert(field.to_string(), cache);
        Ok(())
    }

    pub fn find_by_cached_field(&self, field: &str, value: &str) -> Option<Vec<&Document<T>>> {
        if let Some(cache) = self.cached_fields.get(field) {
            if let Some(ids) = cache.get(&value.to_string()) {
                return Some(
                    self.documents.iter()
                        .filter(|doc| ids.contains(&doc._id))
                        .collect()
                );
            }
        }
        None
    }
}

#[derive(Debug, Default)]
pub struct IndexOptions {
    pub unique: bool,
    pub sparse: bool,
    pub cached: bool,
}
