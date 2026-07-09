use crate::nosql_structural::document::DocumentAccess;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;
use std::collections::HashMap;
use super::references::DbRef;
use crate::error::{DatabaseError, Result};
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
    #[serde(default)]  // Utilise le trait Default si le champ est absent
    cached_fields: CacheMap<String, Vec<Uuid>>, // Ajout du cache
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
            cached_fields: CacheMap::new(), // Initialisation du cache
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
                let existing: Vec<Uuid> = cache.get(&value).unwrap_or_default();
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

#[derive(Debug, Clone)]
pub struct CacheMap<K: Hash + Eq + Clone, V: Clone>(HashMap<String, QueryCache<K, V>>);

impl<K: Hash + Eq + Clone, V: Clone> CacheMap<K, V> {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: QueryCache<K, V>) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&QueryCache<K, V>> {
        self.0.get(key)
    }
}

// Implémentation manuelle de Serialize
impl<K: Hash + Eq + Clone + Serialize, V: Clone + Serialize> Serialize for CacheMap<K, V> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let map: HashMap<String, Vec<(K, V)>> = self.0.iter()
            .map(|(k, v)| (k.clone(), v.iter()))
            .collect();
        map.serialize(serializer)
    }
}

// Implémentation manuelle de Deserialize
impl<'de, K, V> Deserialize<'de> for CacheMap<K, V>
where
    K: Hash + Eq + Clone + Serialize + Deserialize<'de>,
    V: Clone + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_map = HashMap::<String, Vec<(K, V)>>::deserialize(deserializer)?;
        let mut cache_map = HashMap::new();
        
        for (key, items) in raw_map {
            let cache = QueryCache::new(NonZeroUsize::new(1000).unwrap());
            for (k, v) in items {
                cache.insert(k, v);
            }
            cache_map.insert(key, cache);
        }
        
        Ok(CacheMap(cache_map))
    }
}

impl<K: Hash + Eq + Clone, V: Clone> Default for CacheMap<K, V> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}
