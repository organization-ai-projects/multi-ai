use crate::brain::persistent::links::MemoryLink;
use std::collections::HashMap;
use uuid::Uuid;

pub struct LinkCache {
    cache: HashMap<Uuid, MemoryLink>,
    dirty: bool,
}

impl LinkCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            dirty: false,
        }
    }

    pub fn get(&mut self, uuid: &Uuid) -> Option<&MemoryLink> {
        if !self.cache.contains_key(uuid) {
            if let Ok(link) = MemoryLink::load_from_bin(&format!("links/{}.bin", uuid).into()) {
                self.cache.insert(*uuid, link);
            }
        }
        self.cache.get(uuid)
    }

    // ...methods similaires à NodeCache...
}
