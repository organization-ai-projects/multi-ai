use chrono::{DateTime, Utc};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize)]
pub struct BrainMetrics {
    pub total_nodes: usize,
    pub total_links: usize,
    pub memory_usage: MemoryUsage,
    pub learning_stats: LearningStats,
    pub goal_completion: HashMap<String, f32>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct MemoryUsage {
    pub nodes_memory: u64,
    pub links_memory: u64,
    pub artifacts_memory: u64,
    pub episodes_memory: u64,
}

#[derive(Serialize)]
pub struct LearningStats {
    pub successful_mutations: u64,
    pub failed_mutations: u64,
    pub average_score: f32,
    pub best_score: f32,
    pub patterns_discovered: usize,
}

impl BrainMetrics {
    pub fn new() -> Self {
        Self {
            total_nodes: 0,
            total_links: 0,
            memory_usage: MemoryUsage {
                nodes_memory: 0,
                links_memory: 0,
                artifacts_memory: 0,
                episodes_memory: 0,
            },
            learning_stats: LearningStats {
                successful_mutations: 0,
                failed_mutations: 0,
                average_score: 0.0,
                best_score: 0.0,
                patterns_discovered: 0,
            },
            goal_completion: HashMap::new(),
            timestamp: Utc::now(),
        }
    }

    pub fn record_mutation(&mut self, success: bool, score: f32) {
        if success {
            self.learning_stats.successful_mutations += 1;
        } else {
            self.learning_stats.failed_mutations += 1;
        }

        // Mise à jour des scores
        let total = self.learning_stats.successful_mutations + self.learning_stats.failed_mutations;
        self.learning_stats.average_score =
            (self.learning_stats.average_score * (total - 1) as f32 + score) / total as f32;

        if score > self.learning_stats.best_score {
            self.learning_stats.best_score = score;
        }
    }
}
