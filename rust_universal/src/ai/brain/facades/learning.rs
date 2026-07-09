use crate::ai::brain::Brain;

pub struct BrainLearning;

impl BrainLearning {
    pub fn new() -> Self {
        Self
    }

    pub fn reinforce_success(&mut self, concept_id: usize) {
        if let Some(result) = self.was_concept_successful(concept_id) {
            if result {
                println!("✅ Renforcement du concept réussi : {}", concept_id);
                Brain::get_mut().add_link(concept_id, concept_id, 1.0);
            } else {
                println!("❌ Concept échoué ignoré : {}", concept_id);
            }
        } else {
            eprintln!("⚠️ Aucun résultat trouvé pour le concept ID {}", concept_id);
        }
    }

    pub fn associate(&mut self, from: usize, to: usize, weight: f32) {
        Brain::get_mut().add_link(from, to, weight);
    }

    /// Ajoute un lien bidirectionnel entre deux concepts.
    pub fn associate_bidirectional(&mut self, from: usize, to: usize, weight: f32) {
        println!("🔗 Association bidirectionnelle : {} ↔ {} (poids: {})", from, to, weight);
        Brain::get_mut().memory.add_bidirectional_link(from, to, weight);
    }

    /// Ajuste le poids d'un lien existant ou le crée si nécessaire.
    pub fn adjust_association_weight(&mut self, from: usize, to: usize, delta: f32) {
        println!("⚖️ Ajustement du poids du lien : {} → {} (delta: {})", from, to, delta);
        Brain::get_mut().memory.adjust_link_weight(from, to, delta);
    }

    pub fn mark_concept_as_successful(&mut self, concept_id: usize) {
        // Marque un concept comme ayant été renforcé avec succès
        let key = format!("learning_success_{}", concept_id);
        Brain::get_mut()
            .memory
            .store_memory(key, "success".to_string());
        println!("✅ Concept {} marqué comme réussi.", concept_id);
    }

    pub fn weaken_failure(&mut self, concept_id: usize) {
        println!("🔻 Affaiblissement du concept échoué : {}", concept_id);
        Brain::get_mut().add_link(concept_id, concept_id, -1.0);
    }

    pub fn mark_concept_as_failed(&mut self, concept_id: usize) {
        // Marque un concept comme ayant échoué
        let key = format!("learning_failure_{}", concept_id);
        Brain::get_mut()
            .memory
            .store_memory(key, "failure".to_string());
        println!("❌ Concept {} marqué comme échoué.", concept_id);
    }

    pub fn was_concept_successful(&self, concept_id: usize) -> Option<bool> {
        // Vérifie si un concept a été marqué comme réussi
        let key = format!("learning_success_{}", concept_id);
        if let Some(value) = Brain::get_mut().memory.retrieve_memory(&key) {
            return Some(value == "success");
        }
        None
    }

    /// Vérifie si un lien existe entre deux concepts.
    pub fn association_exists(&self, from: usize, to: usize) -> bool {
        Brain::get_mut().memory.link_exists(from, to)
    }

    /// Récupère le poids d'un lien entre deux concepts.
    pub fn get_association_weight(&self, from: usize, to: usize) -> Option<f32> {
        Brain::get_mut().memory.get_link_weight(from, to)
    }
}
