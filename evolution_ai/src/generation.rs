use chrono::{DateTime, Utc};
use ron::ser::to_string_pretty; // Ajout de l'import correct
use serde::Serialize;
use std::fs;

use crate::ai_manager::AIManager;
use crate::archives::ArchiveManager; // Correction de l'import

#[derive(Serialize)]
pub struct GenerationStats {
    pub generation: usize,
    pub timestamp: DateTime<Utc>,
    pub population_size: usize,
    pub deaths: usize,
    pub best_performer: f64,
    pub avg_performance: f64,
}

pub struct Generation {
    pub id: usize,
    pub run_dir: String,
}

impl Generation {
    pub fn new(run_dir: String, id: usize) -> Self {
        Self { run_dir, id }
    }

    pub fn create_stats(
        &self,
        population_size: usize,
        deaths: usize,
        performers: &[(String, f64)],
    ) -> GenerationStats {
        GenerationStats {
            generation: self.id,
            timestamp: Utc::now(),
            population_size,
            deaths,
            best_performer: performers
                .iter()
                .map(|(_, score)| *score)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or(0.0),
            avg_performance: if !performers.is_empty() {
                performers.iter().map(|(_, score)| *score).sum::<f64>() / performers.len() as f64
            } else {
                0.0
            },
        }
    }

    pub fn save_stats(&self, stats: &GenerationStats) -> std::io::Result<()> {
        let stats_dir = format!("{}/stats", self.run_dir);
        fs::create_dir_all(&stats_dir)?;

        let stats_path = format!("{}/generation_{:05}.ron", stats_dir, self.id);
        let ron_string = to_string_pretty(stats, ron::ser::PrettyConfig::default())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        fs::write(stats_path, ron_string)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }

    pub fn archive(&self, manager: &AIManager) -> std::io::Result<()> {
        let archive_manager = ArchiveManager::new(self.run_dir.clone());
        archive_manager.archive_generation(self.id, manager.get_top_performers(5))
    }
}
