use lru::LruCache;
use serde::{Deserialize, Serialize}; // Import correct de serde
use std::num::NonZeroUsize;
use std::sync::Arc;
use parking_lot::RwLock;
use std::hash::Hash;

#[derive(Debug, Clone)]
pub struct QueryCache<K: Hash + Eq, V> {
    #[serde(skip_serializing, skip_deserializing)] // Attribut correctement reconnu
    cache: Arc<RwLock<LruCache<K, V>>>,
}

impl<K, V> QueryCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(capacity: NonZeroUsize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let guard = self.cache.read();
        guard.peek(key).cloned() // Utilise peek au lieu de get pour éviter la mutabilité
    }

    pub fn insert(&self, key: K, value: V) {
        let mut guard = self.cache.write();
        guard.put(key, value);
    }

    pub fn iter(&self) -> Vec<(K, V)> {
        let guard = self.cache.read();
        guard.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}
