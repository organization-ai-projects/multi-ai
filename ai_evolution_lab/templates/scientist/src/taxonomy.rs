use std::collections::HashMap;

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
        // Identifie des patterns communs
        let traits = self.identify_traits(observation);
        let category = self.determine_category(&traits);
        
        // Classe le spécimen
        let specimen = Specimen {
            id: observation.specimen_id.clone(),
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
}
