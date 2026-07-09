use std::collections::HashMap;
use crate::observation::Observation;

#[derive(Debug)]
pub struct Specimen {
    pub id: String,
    pub category: String,
    pub traits: Vec<String>,
    pub observed_behaviors: Vec<String>,
}

pub struct Classification {
    categories: HashMap<String, Vec<Specimen>>,
    known_traits: HashMap<String, u32>,
}

impl Classification {
    pub fn new() -> Self {
        Self {
            categories: HashMap::new(),
            known_traits: HashMap::new(),
        }
    }

    pub fn categorize(&mut self, observation: &Observation) {
        let traits = self.identify_traits(observation);
        let category = self.determine_category(&traits);
        
        let specimen = Specimen {
            id: observation.specimen_id.to_string(),
            category: category.clone(),
            traits,
            observed_behaviors: observation.behavior
                .as_ref()
                .map(|b| vec![b.clone()])
                .unwrap_or_default(),
        };

        self.categories.entry(category)
            .or_default()
            .push(specimen);
    }

    fn identify_traits(&mut self, observation: &Observation) -> Vec<String> {
        let mut traits = Vec::new();
        
        // Analyse du code source
        if let Some(code) = &observation.source_code {
            // Traits structurels
            if code.contains("struct") {
                traits.push("uses_structures".to_string());
            }
            if code.contains("impl") {
                traits.push("has_implementations".to_string());
            }

            // Traits comportementaux
            if code.contains("loop") {
                traits.push("cyclic_behavior".to_string());
            }
            if code.contains("match") {
                traits.push("pattern_matching".to_string());
            }
        }

        // Traits de comportement
        if observation.is_alive {
            traits.push("survived_execution".to_string());
        }
        if observation.execution_time_ms > 1000 {
            traits.push("long_lived".to_string());
        }

        traits
    }

    fn determine_category(&mut self, traits: &[String]) -> String {
        // Mise à jour des statistiques
        for trait_name in traits {
            *self.known_traits.entry(trait_name.clone()).or_insert(0) += 1;
        }

        // Catégorisation basée sur les traits dominants
        if traits.contains(&"long_lived".to_string()) && traits.contains(&"cyclic_behavior".to_string()) {
            "persistent".to_string()
        } else if traits.len() > 3 {
            "complex".to_string()
        } else {
            "basic".to_string()
        }
    }

    pub fn get_category_specimens(&self, category: &str) -> Option<&Vec<Specimen>> {
        self.categories.get(category)
    }

    pub fn get_trait_frequency(&self, trait_name: &str) -> u32 {
        *self.known_traits.get(trait_name).unwrap_or(&0)
    }
}
