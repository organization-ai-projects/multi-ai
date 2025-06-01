use std::collections::BTreeMap;
use uuid::Uuid;
use super::super::error::Result;

pub struct BTreeIndex<K: Ord> {
    tree: BTreeMap<K, Vec<Uuid>>,
    unique: bool,
}

impl<K: Ord + Clone> BTreeIndex<K> {
    pub fn new(unique: bool) -> Self {
        Self {
            tree: BTreeMap::new(),
            unique,
        }
    }

    pub fn insert(&mut self, key: K, doc_id: Uuid) -> Result<()> {
        if self.unique && self.tree.contains_key(&key) {
            return Err(super::super::error::Error::UniqueConstraintViolation(
                "Index must be unique".into()
            ));
        }

        self.tree
            .entry(key)
            .or_insert_with(Vec::new)
            .push(doc_id);
        Ok(())
    }

    pub fn find(&self, key: &K) -> Option<&Vec<Uuid>> {
        self.tree.get(key)
    }

    pub fn range(&self, start: &K, end: &K) -> Vec<Uuid> {
        self.tree
            .range(start..=end)
            .flat_map(|(_, ids)| ids)
            .copied()
            .collect()
    }
}
