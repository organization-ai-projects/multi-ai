use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Arc;
use parking_lot::RwLock;

pub struct QueryCache<K, V> {
    cache: Arc<RwLock<LruCache<K, V>>>,
}

impl<K: Clone + Eq + std::hash::Hash, V: Clone> QueryCache<K, V> {
    pub fn new(capacity: NonZeroUsize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.read().get(key).cloned()
    }

    pub fn insert(&self, key: K, value: V) {
        self.cache.write().put(key, value);
    }
}
