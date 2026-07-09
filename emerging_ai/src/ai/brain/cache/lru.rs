use lru::LruCache;
use std::hash::Hash;

pub struct SizedCache<K, V> {
    cache: LruCache<K, V>,
    max_size: usize,
}

impl<K: Hash + Eq, V> SizedCache<K, V> {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: LruCache::new(max_size),
            max_size,
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        if self.cache.len() >= self.max_size {
            // Éviction selon politique LRU gérée par LruCache
            self.cache.pop_lru();
        }
        self.cache.put(key, value)
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        self.cache.pop(key)
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
