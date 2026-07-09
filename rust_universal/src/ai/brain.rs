use crate::ai::memory::persist_memory_graph::PersistentMemoryGraph;

// ---------- Implémentation principale de Brain ----------
pub struct Brain {
    memory: PersistentMemoryGraph,
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

    pub fn retrieve_memory(&self, key: &str) -> Option<&String> {
        self.memory.retrieve_memory(key)
    }

    pub fn list_short_term_memory(&self) -> Vec<&String> {
        self.memory.list_short_term()
    }

    pub fn list_medium_term_memory(&self) -> Vec<&String> {
        self.memory.list_medium_term()
    }

    pub fn list_long_term_memory(&self) -> Vec<&String> {
        self.memory.list_long_term()
    }

    pub fn update_to_short_term(&mut self, key: &str) {
        self.memory.update_to_short_term(key);
    }

    pub fn update_to_medium_term(&mut self, key: &str) {
        self.memory.update_to_medium_term(key);
    }

    pub fn update_to_long_term(&mut self, key: &str) {
        self.memory.update_to_long_term(key);
    }

    // Méthodes de création des façades
    pub fn perception(&mut self) -> BrainPerception {
        BrainPerception { brain: self }
    }

    pub fn cognition(&mut self) -> BrainCognition {
        BrainCognition { brain: self }
    }

    pub fn learning(&mut self) -> BrainLearning {
        BrainLearning { brain: self }
    }

    pub fn motivation(&mut self) -> BrainMotivation {
        BrainMotivation { brain: self }
    }

    // Ces méthodes doivent être mises à jour pour utiliser les nouvelles méthodes spécifiques
    pub fn store_short_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.memory.store_short_term(key, value);
    }

    pub fn store_medium_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.memory.store_medium_term(key, value);
    }

    pub fn store_long_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.memory.store_long_term(key, value);
    }
}

// ---------- Façades ----------

pub struct BrainPerception<'a> {
    pub(crate) brain: &'a mut Brain,
}

impl<'a> BrainPerception<'a> {
    pub fn observe_text(&mut self, text: &str) {
        self.brain.memory.add_node(1, text);
    }
}

pub struct BrainCognition<'a> {
    pub(crate) brain: &'a mut Brain,
}

impl<'a> BrainCognition<'a> {
    pub fn associate(&mut self, from: usize, to: usize, weight: f32) {
        self.brain.memory.add_link(from, to, weight);
    }

    pub fn validate(&self) -> bool {
        self.brain.memory.is_valid()
    }
}

pub struct BrainLearning<'a> {
    pub(crate) brain: &'a mut Brain,
}

impl<'a> BrainLearning<'a> {
    pub fn reinforce_success(&mut self, concept_id: usize) {
        self.brain.memory.add_link(concept_id, concept_id, 1.0);
    }

    pub fn associate(&mut self, from: usize, to: usize, weight: f32) {
        self.brain.memory.add_link(from, to, weight);
    }
}

pub struct BrainMotivation<'a> {
    pub(crate) brain: &'a mut Brain,
}

impl<'a> BrainMotivation<'a> {
    pub fn evaluate(&self) {
        println!("Motivation évaluée.");
    }
}
