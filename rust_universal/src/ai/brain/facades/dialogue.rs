use crate::ai::brain::Brain;

pub struct BrainDialogue;

impl BrainDialogue {
    pub fn new() -> Self {
        Self
    }

    // Transforme l'intention "stocker un dialogue" en manipulation mémoire
    pub fn store_dialogue(&mut self, text: &str) {
        Brain::get_mut().add_node(1, text);
    }

    // Transforme l'intention "lier deux dialogues" en manipulation mémoire
    pub fn link_dialogues(&mut self, from: usize, to: usize, relevance: f32) {
        Brain::get_mut().add_link(from, to, relevance);
    }

    /// Lie deux dialogues de manière bidirectionnelle.
    pub fn link_dialogues_bidirectional(&mut self, from: usize, to: usize, relevance: f32) {
        println!("🔗 Lien bidirectionnel entre dialogues : {} ↔ {} (pertinence: {})", from, to, relevance);
        Brain::get_mut().memory.add_bidirectional_link(from, to, relevance);
    }

    /// Récupère une réponse depuis la mémoire en fonction de l'entrée utilisateur.
    pub fn retrieve_response(&self, input: &str) -> Option<String> {
        let key = format!("response_{}", input.to_lowercase());
        Brain::get_mut().memory.retrieve_memory(&key).cloned()
    }

    /// Stocke une réponse dans la mémoire pour une entrée donnée.
    pub fn store_response(&mut self, input: &str, response: &str) {
        let key = format!("response_{}", input.to_lowercase());
        Brain::get_mut().memory.store_memory(key, response.to_string());
    }
}
