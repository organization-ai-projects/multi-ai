use chrono::{DateTime, Utc};
use ron::ser::to_string_pretty;
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct GenerationStats {
    pub generation: usize,
    pub timestamp: DateTime<Utc>,
    pub population_size: usize,
    pub best_performance: f64,
    pub avg_performance: f64,
}

pub struct Generation {
    pub id: usize,
    pub created_at: DateTime<Utc>,
}

impl Generation {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            created_at: Utc::now(),
        }
    }

    pub fn create_stats(
        &self,
        population_size: usize,
        best_performance: f64,
        avg_performance: f64,
    ) -> GenerationStats {
        GenerationStats {
            generation: self.id,
            timestamp: self.created_at,
            population_size,
            best_performance,
            avg_performance,
        }
    }
}
