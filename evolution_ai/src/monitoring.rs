use crate::species::{SpeciesStats, SpeciesType};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Clone)] // Ajout de Clone
pub struct EvolutionSnapshot {
    pub timestamp: DateTime<Utc>,
    pub species_stats: HashMap<SpeciesType, SpeciesStats>,
    pub global_diversity: f64,
    pub generation: usize, // Ajout du mot-clé pub
}

pub struct Monitor {
    log_dir: String,
    current_generation: usize,
}

impl Monitor {
    pub fn new() -> Self {
        let log_dir = format!("logs/evolution_{}", Utc::now().format("%Y%m%d_%H%M%S"));
        fs::create_dir_all(&log_dir).unwrap();
        Self {
            log_dir,
            current_generation: 0,
        }
    }

    pub fn log_extinction(&self, species: &SpeciesType) {
        let log_path = format!("{}/extinctions.log", self.log_dir);
        let entry = format!("[{}] Species {:?} extinct\n", Utc::now(), species,);
        fs::write(log_path, entry).unwrap_or_default();
    }

    pub fn save_snapshot(&mut self, snapshot: EvolutionSnapshot) {
        let path = format!("{}/gen_{:05}.json", self.log_dir, self.current_generation);
        if let Ok(json) = serde_json::to_string_pretty(&snapshot) {
            fs::write(path, json).unwrap_or_default();
        }
        self.current_generation += 1;
    }

    pub fn calculate_diversity(performers: &[(String, f64)]) -> f64 {
        if performers.is_empty() {
            return 0.0;
        }

        let scores: Vec<f64> = performers.iter().map(|(_, score)| *score).collect();
        let mean = scores.iter().sum::<f64>() / scores.len() as f64;
        let variance = scores.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / scores.len() as f64;

        variance.sqrt() // écart-type comme mesure de diversité
    }
}
