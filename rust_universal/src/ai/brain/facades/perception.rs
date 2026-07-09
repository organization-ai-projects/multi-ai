use crate::ai::brain::Brain;

pub struct BrainPerception;

impl BrainPerception {
    pub fn new() -> Self {
        Self
    }

    pub fn observe_text(&mut self, text: &str) {
        Brain::get_mut().add_node(1, text);
    }
}
