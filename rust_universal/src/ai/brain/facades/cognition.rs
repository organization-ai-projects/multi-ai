use crate::ai::brain::Brain;

pub struct BrainCognition;

impl BrainCognition {
    pub fn new() -> Self {
        Self
    }

    pub fn associate(&mut self, from: usize, to: usize, weight: f32) {
        Brain::get_mut().add_link(from, to, weight);
    }

    pub fn validate(&self) -> bool {
        Brain::get_mut().is_memory_valid()
    }

    /// Ajuste le poids d'une association cognitive.
    pub fn adjust_cognitive_association(&mut self, from: usize, to: usize, delta: f32) {
        println!(
            "⚖️ Ajustement du poids cognitif : {} → {} (delta: {})",
            from, to, delta
        );
        Brain::get_mut().memory.adjust_link_weight(from, to, delta);
    }
}
