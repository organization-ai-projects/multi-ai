use crate::common::error::MemoryResult;
use crate::memory::storage_graph_memory::StorageGraphMemory;

/// Façade officielle pour manipuler une IA à mémoire graphique.
/// Contient `StorageGraphMemory` qui gère la RAM + persistance.
pub struct GraphMemoryStorageAPI<'a> {
    internal: StorageGraphMemory<'a>,
}

impl<'a> GraphMemoryStorageAPI<'a> {
    pub fn new(storage: StorageGraphMemory<'a>) -> Self {
        Self { internal: storage }
    }

    // ------------------- PERSISTENCE -------------------

    pub fn load(&mut self, ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        self.internal.load(ia_name, root)
    }

    pub fn save(&self, ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        self.internal.save(ia_name, root)
    }

    pub fn delete(ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        StorageGraphMemory::delete_static(ia_name, root)
    }

    // ------------------- MÉMOIRE -------------------

    pub fn create_node(&mut self, id: &str, label: &str) -> String {
        self.internal.create_and_add_node(id, label)
    }

    pub fn add_link(&mut self, from: &str, to: &str, label: Option<String>, weight: Option<f32>) -> bool {
        self.internal.create_and_add_link(from, to, label, weight)
    }

    pub fn node_exists(&self, id: &str) -> bool {
        self.internal.node_exists(id)
    }

    pub fn get_node_label(&self, id: &str) -> Option<String> {
        self.internal.get_node_label(id)
    }

    pub fn get_node_attribute(&self, id: &str, key: &str) -> Option<String> {
        self.internal.get_node_attribute(id, key)
    }

    pub fn get_all_node_ids(&self) -> Vec<String> {
        self.internal.get_all_node_ids()
    }

    pub fn get_links_for_node(&self, id: &str) -> Vec<(String, String, Option<String>, Option<f32>)> {
        self.internal.get_links_for_node(id)
    }

    pub fn get_links_for_all_nodes(&self) -> Vec<(String, String, Option<String>, Option<f32>)> {
        self.internal.get_links_for_all_nodes()
    }

    pub fn start_node(&mut self, id: &str, label: &str) -> String {
        self.internal.create_node_begin(id, label)
    }

    pub fn add_attribute(&mut self, id: &str, key: &str, value: &str) -> bool {
        self.internal.create_node_add_attribute(id, key, value)
    }

    pub fn finish_node(&mut self, id: &str) -> Option<String> {
        self.internal.create_node_finish(id)
    }

    pub fn add_tag(&mut self, id: &str, tag: &str) -> bool {
        self.internal.add_tag_to_node(id, tag)
    }

    pub fn has_tag(&self, id: &str, tag: &str) -> bool {
        self.internal.node_has_tag(id, tag)
    }
}
