use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default)]
pub struct NatureMemory {
    pub known_combinations: HashMap<String, CombinationKnowledge>,
    pub observed_lifetimes: Vec<u64>,
    pub lethal_patterns: Vec<String>,
    pub successful_patterns: Vec<String>,
    pub total_observations: u64
}

#[derive(Serialize, Deserialize)]
pub struct CombinationKnowledge {
    pub occurrences: u32,
    pub success_rate: f32,
    pub avg_lifetime: u64,
    pub last_seen: i64,
}

impl NatureMemory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_observation(&mut self, code: &str, survived: bool, lifetime: u64) {
        let now = chrono::Utc::now().timestamp();
        
        let entry = self.known_combinations
            .entry(code.to_string())
            .or_insert(CombinationKnowledge {
                occurrences: 0,
                success_rate: 0.0,
                avg_lifetime: 0,
                last_seen: now
            });

        entry.occurrences += 1;
        entry.last_seen = now;
        entry.avg_lifetime = (entry.avg_lifetime + lifetime) / 2;
        
        let new_success_rate = if survived {
            (entry.success_rate * (entry.occurrences - 1) as f32 + 1.0) / entry.occurrences as f32
        } else {
            (entry.success_rate * (entry.occurrences - 1) as f32) / entry.occurrences as f32
        };
        entry.success_rate = new_success_rate;

        self.observed_lifetimes.push(lifetime);
        self.total_observations += 1;

        if survived {
            self.successful_patterns.push(code.to_string());
        } else {
            self.lethal_patterns.push(code.to_string());
        }
    }

    pub fn save_state(&self, path: &str) {
        // Sauvegarde au format RON
        if let Ok(state) = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()) {
            if let Err(e) = std::fs::write(format!("{}.ron", path), state) {
                eprintln!("Erreur lors de la sauvegarde de l'état (RON): {}", e);
            }
        }

        // Sauvegarde au format binaire
        if let Ok(state) = bincode::serialize(self) {
            if let Err(e) = std::fs::write(format!("{}.bin", path), state) {
                eprintln!("Erreur lors de la sauvegarde de l'état (binaire): {}", e);
            }
        }
    }
}
