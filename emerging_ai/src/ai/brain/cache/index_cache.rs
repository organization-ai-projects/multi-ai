use crate::brain::persistent::index::GlobalIndex;
use uuid::Uuid;

pub struct IndexCache {
    index: GlobalIndex,
    dirty: bool,
}

impl IndexCache {
    pub fn new() -> Self {
        Self {
            index: GlobalIndex::new(),
            dirty: false,
        }
    }

    pub fn add_node(&mut self, uuid: Uuid, timestamp: u64) {
        self.index.add_node(uuid, timestamp);
        self.dirty = true;
    }

    pub fn get(&self, uuid: &Uuid) -> Option<u64> {
        self.index
            .nodes
            .get(uuid)
            .or_else(|| self.index.links.get(uuid))
            .or_else(|| self.index.logs.get(uuid))
            .copied()
    }

    pub fn list(&self) -> Vec<Uuid> {
        let mut uuids = Vec::new();
        uuids.extend(self.index.nodes.keys());
        uuids.extend(self.index.links.keys());
        uuids.extend(self.index.logs.keys());
        uuids
    }

    pub fn insert(&mut self, uuid: Uuid, timestamp: u64) {
        self.index.nodes.insert(uuid, timestamp);
        self.dirty = true;
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.dirty {
            self.index.save_to_file("index.ron")?;
            self.dirty = false;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.index.nodes.clear();
        self.index.links.clear();
        self.index.logs.clear();
        self.dirty = false;
    }
}
