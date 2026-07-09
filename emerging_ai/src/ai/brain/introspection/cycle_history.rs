use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize)]
pub struct CycleSnapshot {
    pub cycle_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub actions: Vec<String>,
    pub mutations: Vec<String>,
    pub scores: Vec<f32>,
    pub memory_delta: MemoryDelta,
    pub metrics: CycleMetrics,
}

#[derive(Serialize)]
pub struct MemoryDelta {
    pub nodes_added: Vec<Uuid>,
    pub nodes_modified: Vec<Uuid>,
    pub links_added: Vec<Uuid>,
}

#[derive(Serialize)]
pub struct CycleMetrics {
    pub duration: std::time::Duration,
    pub ram_usage: u64,
    pub cpu_usage: f32,
    pub artifacts_produced: u32,
}

impl CycleSnapshot {
    pub fn new(cycle_id: Uuid) -> Self {
        Self {
            cycle_id,
            timestamp: Utc::now(),
            actions: Vec::new(),
            mutations: Vec::new(),
            scores: Vec::new(),
            memory_delta: MemoryDelta {
                nodes_added: Vec::new(),
                nodes_modified: Vec::new(),
                links_added: Vec::new(),
            },
            metrics: CycleMetrics {
                duration: std::time::Duration::default(),
                ram_usage: 0,
                cpu_usage: 0.0,
                artifacts_produced: 0,
            }
        }
    }

    pub async fn save(&self) -> std::io::Result<()> {
        let path = format!("monitoring/cycles/{}/{}.json", 
            self.timestamp.format("%Y-%m-%d"),
            self.cycle_id
        );
        
        tokio::fs::create_dir_all(
            std::path::Path::new(&path).parent().unwrap()
        ).await?;

        tokio::fs::write(
            path,
            serde_json::to_string_pretty(self)?
        ).await?;

        Ok(())
    }

    pub async fn load(date: &str, cycle_id: Uuid) -> std::io::Result<Self> {
        let path = format!("monitoring/cycles/{}/{}.json", date, cycle_id);
        let data = tokio::fs::read_to_string(path).await?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn add_action(&mut self, action: String) {
        self.actions.push(action);
    }

    pub fn add_mutation(&mut self, mutation: String) {
        self.mutations.push(mutation);
    }

    pub fn add_score(&mut self, score: f32) {
        self.scores.push(score);
    }

    pub fn update_metrics(&mut self, metrics: CycleMetrics) {
        self.metrics = metrics;
    }

    pub fn analyze_mutations(&self) -> Vec<(String, f32)> {
        self.mutations
            .iter()
            .map(|m| (m.clone(), self.calculate_mutation_impact(m)))
            .collect()
    }

    fn calculate_mutation_impact(&self, mutation: &str) -> f32 {
        // Calcul de l'impact basé sur les scores avant/après
        // Implementation...
        0.0
    }
}