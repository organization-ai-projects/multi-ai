use crate::brain::memory_episode::MemoryEpisode;
use serde_json;
use std::fs::File;
use std::collections::HashMap;

pub struct EpisodeManager {
    pub episodes: Vec<MemoryEpisode>,
}

impl EpisodeManager {
    pub fn new() -> Self {
        EpisodeManager {
            episodes: Vec::new(),
        }
    }

    pub fn add_episode(&mut self, episode: MemoryEpisode) {
        self.episodes.push(episode);
    }

    pub fn save_episodes(&self, path: &str) {
        let file = File::create(path).expect("Impossible de créer le fichier des épisodes");
        serde_json::to_writer(file, &self.episodes).expect("Impossible de sérialiser les épisodes");
    }

    pub fn load_episodes(path: &str) -> Vec<MemoryEpisode> {
        let file = File::open(path).expect("Impossible d'ouvrir le fichier des épisodes");
        serde_json::from_reader(file).expect("Impossible de désérialiser les épisodes")
    }

    pub fn save_partitioned_episodes(&self, base_path: &str) {
        let mut episodes_by_date: HashMap<String, Vec<&MemoryEpisode>> = HashMap::new();

        for episode in &self.episodes {
            let date = chrono::NaiveDateTime::from_timestamp(episode.timestamp as i64, 0)
                .format("%Y-%m-%d")
                .to_string();
            episodes_by_date.entry(date).or_default().push(episode);
        }

        for (date, episodes) in episodes_by_date {
            let path = format!("{}/episodes_{}.json", base_path, date);
            let file = File::create(&path).expect("Impossible de créer le fichier des épisodes partitionnés");
            serde_json::to_writer(file, &episodes).expect("Impossible de sérialiser les épisodes");
        }
    }

    pub fn generate_analytics(&self) -> String {
        let mut strategy_stats: HashMap<String, (u32, u32)> = HashMap::new(); // (succès, échecs)

        for episode in &self.episodes {
            let entry = strategy_stats.entry(format!("{:?}", episode.strategy)).or_insert((0, 0));
            if episode.success {
                entry.0 += 1; // Succès
            } else {
                entry.1 += 1; // Échec
            }
        }

        let mut report = String::new();
        report.push_str("Statistiques des stratégies :\n");
        for (strategy, (successes, failures)) in strategy_stats {
            report.push_str(&format!(
                "Stratégie : {} | Succès : {} | Échecs : {}\n",
                strategy, successes, failures
            ));
        }

        report
    }
}
