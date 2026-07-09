use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesStats {
    pub total_generations: usize,
    pub peak_performance: f64,
    pub last_seen: DateTime<Utc>,
    pub population_size: usize,
    pub mutation_rate_boost: f64,
}

impl SpeciesStats {
    pub fn new() -> Self {
        Self {
            total_generations: 0,
            peak_performance: 0.0,
            last_seen: Utc::now(),
            population_size: 0,
            mutation_rate_boost: 1.0,
        }
    }
}
