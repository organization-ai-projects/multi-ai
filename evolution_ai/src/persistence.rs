use crate::{
    ai_manager::AIManager, archives::ArchiveManager, species::{Lineage, SpeciesStats, SpeciesType}
};
use chrono::{DateTime, Utc};
use ron::ser::to_string_pretty; // Import explicite
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Checkpoint {
    pub generation: usize,
    pub timestamp: DateTime<Utc>,
    pub species_stats: HashMap<SpeciesType, SpeciesStats>,
    pub lineages: HashMap<String, Lineage>,
    pub performers: Vec<(String, f64)>,
}

pub struct PersistenceManager {
    pub base_dir: String,
    pub keep_generations: usize,
}

impl PersistenceManager {
    pub fn new(base_dir: String, keep_generations: usize) -> Self {
        Self {
            base_dir,
            keep_generations,
        }
    }

    pub fn save_checkpoint(
        &self,
        generation: usize,
        manager: &AIManager,
        species_stats: &HashMap<SpeciesType, SpeciesStats>,
    ) -> std::io::Result<()> {
        let checkpoint = Checkpoint {
            generation,
            timestamp: Utc::now(),
            species_stats: species_stats.clone(),
            lineages: manager.get_lineages(),
            performers: manager.get_performers(),
        };

        let checkpoint_path = format!("{}/checkpoint.ron", self.base_dir);
        fs::write(
            checkpoint_path,
            to_string_pretty(&checkpoint, Default::default())
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
        )?;
        Ok(())
    }

    pub fn load_checkpoint(&self) -> Option<Checkpoint> {
        let checkpoint_path = format!("{}/checkpoint.ron", self.base_dir);
        fs::read_to_string(checkpoint_path)
            .ok()
            .and_then(|content| ron::from_str(&content).ok())
    }

    pub fn archive_generation(
        &self,
        generation: usize,
        ai_paths: Vec<std::path::PathBuf>,
    ) -> std::io::Result<()> {
        let archive_manager = ArchiveManager::new(self.base_dir.clone());
        archive_manager.archive_generation(generation, ai_paths)
    }
}
