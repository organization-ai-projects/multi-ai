use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
pub struct DiscoveredMolecule {
    pub content: String,
    pub success_count: u32,
    pub failure_count: u32,
    pub last_context: String,
}

#[derive(Serialize, Deserialize)]
pub struct ObservedMolecule {
    pub content: String,
    pub observed_contexts: Vec<String>,
    pub viable_count: u32,
    pub total_uses: u32
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentMemory {
    pub discovered_molecules: HashMap<String, u32>,
    pub observed_combinations: Vec<String>,
    pub dissection_results: Vec<String>,
    pub total_observations: u32,
    pub total_dissections: u32
}

impl ExperimentMemory {
    pub fn new() -> Self {
        Self {
            discovered_molecules: HashMap::new(),
            observed_combinations: Vec::new(),
            dissection_results: Vec::new(),
            total_observations: 0,
            total_dissections: 0
        }
    }

    pub fn record_discovery(&mut self, molecule: String) {
        *self.discovered_molecules.entry(molecule).or_insert(0) += 1;
    }
}

#[derive(Serialize, Deserialize)]
pub struct VisualMemory {
    // Représentation visuelle des séquences génétiques observées
    genome_patterns: HashMap<String, Vec<u8>>, // génome -> représentation visuelle
    success_patterns: Vec<Vec<u8>>,           // motifs visuels qui ont fonctionné
    lethal_patterns: Vec<Vec<u8>>,            // motifs visuels mortels
    current_knowledge: u32,                    // niveau de compréhension
}

#[derive(Serialize, Deserialize)]
pub struct GenomeSequence {
    sequence: Vec<String>,           // molécules dans l'ordre
    visual_pattern: Vec<u8>,         // représentation visuelle
    survival_rate: f32,             // taux de survie observé
    discovered_at: u64,             // quand on l'a découvert
    last_observation: u64,          // dernière fois qu'on l'a vu
}

impl VisualMemory {
    pub fn new() -> Self {
        Self {
            genome_patterns: HashMap::new(),
            success_patterns: Vec::new(),
            lethal_patterns: Vec::new(),
            current_knowledge: 0,
        }
    }

    pub fn observe_genome(&mut self, code: &str, survived: bool, lifetime: u64) {
        let visual = self.code_to_visual_pattern(code);
        let sequence = self.extract_genome_sequence(code);

        if survived {
            self.success_patterns.push(visual.clone());
        } else {
            self.lethal_patterns.push(visual.clone());
        }

        self.genome_patterns.insert(sequence.join(""), visual);
        self.update_knowledge();
    }

    pub fn try_genetic_modification(&self, base_genome: &str) -> Option<String> {
        if self.current_knowledge < 10 {
            return None; // Pas assez de connaissances
        }

        let visual = self.code_to_visual_pattern(base_genome);
        
        // Cherche des motifs similaires qui ont survécu
        for success in &self.success_patterns {
            if self.visual_similarity(&visual, success) > 0.8 {
                // Tente une modification basée sur ce pattern
                return Some(self.modify_genome(base_genome, success));
            }
        }

        None
    }

    fn code_to_visual_pattern(&self, code: &str) -> Vec<u8> {
        // Convertit le code en pattern visuel (comme l'oeil humain)
        code.bytes().collect()
    }

    fn visual_similarity(&self, pattern1: &[u8], pattern2: &[u8]) -> f32 {
        // Compare deux patterns visuels (comme le cerveau)
        // ...
        0.5 // temporaire
    }

    fn modify_genome(&self, base: &str, target_pattern: &[u8]) -> String {
        // Modification génétique basée sur la compréhension visuelle
        // ...
        base.to_string() // temporaire
    }

    fn update_knowledge(&mut self) {
        // Plus on observe, plus on comprend
        self.current_knowledge = (self.success_patterns.len() + 
            self.lethal_patterns.len()) as u32;
    }
}

impl ObservedMolecule {
    pub fn new(content: String) -> Self {
        Self {
            content,
            observed_contexts: Vec::new(),
            viable_count: 0,
            total_uses: 0
        }
    }
}
