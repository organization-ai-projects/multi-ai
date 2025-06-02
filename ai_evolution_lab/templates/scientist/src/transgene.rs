use crate::memory::VisualMemory;
use uuid::{Uuid, uuid7};
use std::collections::HashMap;

pub struct ObservedPattern {
    id: Uuid,                    // Identifiant unique du pattern observé
    frequency: u32,              // Nombre de fois observé
    contexts: Vec<Uuid>,         // IDs des contextes où on l'a vu
    effect: HashMap<Uuid, f32>,  // Corrélations avec d'autres patterns
    survival_impact: f32,        // Impact observé sur la survie (-1.0 à 1.0)
}

pub struct GeneticModification {
    source_pattern: Uuid,        // Pattern d'origine observé
    target_pattern: Uuid,        // Pattern qu'on tente de reproduire/modifier
    operation: ModificationType,
    confidence: f32,            
}

pub enum ModificationType {
    Pattern1ToPattern2 {        // Un pattern devient un autre
        from: Uuid,
        to: Uuid
    },
    Combination {               // Fusion de patterns observés
        patterns: Vec<Uuid>
    },
    Partial {                   // Modification partielle observée
        target: Uuid,
        magnitude: f32
    }
}

impl ObservedPattern {
    pub fn new(context: Uuid) -> Self {
        Self {
            id: uuid7(),  // Utilise uuid7() au lieu de new_v4()
            frequency: 1,
            contexts: vec![context],
            effect: HashMap::new(),
            survival_impact: 0.0
        }
    }

    pub fn record_correlation(&mut self, other_pattern: Uuid, impact: f32) {
        let entry = self.effect.entry(other_pattern).or_insert(0.0);
        *entry = (*entry + impact) / 2.0;  // Moyenne glissante
    }
}

impl GeneticModification {
    pub fn try_modify(genome: &str, memory: &VisualMemory) -> Option<Self> {
        // Tente une modification basée sur la mémoire visuelle
        memory.try_genetic_modification(genome).map(|modified| {
            Self {
                source_pattern: Uuid::new_v4(),
                target_pattern: Uuid::new_v4(),
                operation: ModificationType::Substitution {
                    old: "".into(),
                    new: "".into()
                },
                confidence: 0.5
            }
        })
    }
}
