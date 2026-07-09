use crate::brain::persistent::espisode::MemoryEpisode;
use std::collections::HashMap;
use uuid::Uuid;

pub struct EpisodeCache {
    cache: HashMap<Uuid, MemoryEpisode>,
    dirty: bool,
}

impl EpisodeCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            dirty: false,
        }
    }

    pub fn get(&mut self, uuid: &Uuid) -> Option<&MemoryEpisode> {
        if !self.cache.contains_key(uuid) {
            if let Ok(episode) = MemoryEpisode::load_from_bin(&format!("episodes/{}.bin", uuid).into()) {
                self.cache.insert(*uuid, episode);
            }
        }
        self.cache.get(uuid)
    }

    pub fn insert(&mut self, episode: MemoryEpisode) {
        self.cache.insert(episode.uuid, episode);
        self.dirty = true;
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.dirty {
            for episode in self.cache.values() {
                episode.save_to_bin(&format!("episodes/{}.bin", episode.uuid).into())?;
            }
            self.dirty = false;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.dirty = false;
    }
}
