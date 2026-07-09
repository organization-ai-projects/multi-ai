use super::{species::Species, species_stats::SpeciesStats};
use chrono::Utc;
use std::collections::HashMap;

pub struct SpeciesManager {
    species_stats: HashMap<String, SpeciesStats>,
    species_list: HashMap<String, Species>, // Associe un nom à une espèce
}

impl SpeciesManager {
    pub fn new() -> Self {
        Self {
            species_stats: HashMap::new(),
            species_list: HashMap::new(),
        }
    }

    /// Ajoute une nouvelle espèce au gestionnaire.
    pub fn add_species(&mut self, species: Species) {
        self.species_list.insert(species.name.clone(), species);
    }

    /// Met à jour les statistiques des espèces à partir des performances.
    pub fn update_stats(&mut self, performers: &[(String, f64)]) {
        for (species_name, stats) in self.species_stats.iter_mut() {
            stats.last_seen = Utc::now();
            stats.population_size += performers
                .iter()
                .filter(|(name, _)| name == species_name)
                .count();
            stats.peak_performance = stats.peak_performance.max(
                performers
                    .iter()
                    .filter(|(name, _)| name == species_name)
                    .map(|(_, score)| *score)
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0),
            );
        }
    }

    /// Retourne une description de l'espèce.
    pub fn describe_species(&self, species_name: &str) -> Option<String> {
        self.species_list
            .get(species_name)
            .map(|species| species.description())
    }
}
