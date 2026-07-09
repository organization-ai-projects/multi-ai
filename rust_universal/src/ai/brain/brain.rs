use crate::ai::memory::persist_memory_graph::PersistentMemoryGraph;
use once_cell::sync::Lazy;
use std::cell::RefCell;

// ---------- Implémentation principale de Brain ----------
pub struct Brain {
    pub(crate) memory: PersistentMemoryGraph,
}

impl Brain {
    pub fn new() -> Self {
        Self {
            memory: PersistentMemoryGraph::new(),
        }
    }

    pub fn load(&mut self) -> std::io::Result<()> {
        self.memory.load()
    }

    pub fn save(&self) -> std::io::Result<()> {
        self.memory.save()
    }

    // Méthodes publiques pour interagir avec la mémoire
    pub fn add_node(&mut self, node_type: usize, data: &str) {
        self.memory.add_node(node_type, data);
    }

    pub fn add_link(&mut self, from: usize, to: usize, weight: f32) {
        self.memory.add_link(from, to, weight);
    }

    pub fn is_memory_valid(&self) -> bool {
        self.memory.is_valid()
    }

    // Singleton Brain global, accessible uniquement depuis les façades
    static GLOBAL_BRAIN: Lazy<RefCell<Brain>> = Lazy::new(|| RefCell::new(Brain::new()));

    pub fn get_mut() -> std::cell::RefMut<'static, Brain> {
        GLOBAL_BRAIN.borrow_mut()
    }
}