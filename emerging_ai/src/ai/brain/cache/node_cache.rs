use crate::brain::persistent::nodes::MemoryNode;  // ← On utilise persistent/ et non memory/
use std::collections::HashMap;
use uuid::Uuid;

pub struct NodeCache {
    cache: HashMap<Uuid, MemoryNode>,
    dirty: bool,
}

impl NodeCache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            dirty: false,
        }
    }

    pub fn get(&mut self, uuid: &Uuid) -> Option<&MemoryNode> {
        if !self.cache.contains_key(uuid) {
            if let Ok(node) = MemoryNode::load_from_bin(&format!("nodes/{}.bin", uuid).into()) {
                self.cache.insert(*uuid, node);
            }
        }
        self.cache.get(uuid)
    }

    pub fn insert(&mut self, node: MemoryNode) {
        self.cache.insert(node.uuid, node);
        self.dirty = true;
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.dirty {
            for node in self.cache.values() {
                node.save_to_bin(&format!("nodes/{}.bin", node.uuid).into())?;
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
