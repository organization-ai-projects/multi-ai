use ron::ser::to_writer;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read};
use std::fs::OpenOptions;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiEvent {
    pub snapshot_id: String,
    pub impact: String,
    pub changelog: String,
    pub decision: String,
    pub human_validated: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiGraph {
    pub nodes: HashMap<String, AiEvent>, // ID de l'événement -> événement
    pub links: HashSet<(String, String)>, // Liens entre événements (ID -> ID)
}

impl AiGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashSet::new(),
        }
    }

    pub fn add_event(&mut self, event: AiEvent) {
        self.nodes.insert(event.snapshot_id.clone(), event);
    }

    pub fn add_link(&mut self, from: &str, to: &str) {
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            self.links.insert((from.to_string(), to.to_string()));
        }
    }

    pub fn find_similar_cases(&self, snapshot_id: &str) -> Vec<&AiEvent> {
        self.links
            .iter()
            .filter_map(|(from, to)| {
                if from == snapshot_id {
                    self.nodes.get(to)
                } else if to == snapshot_id {
                    self.nodes.get(from)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Prédit une décision basée sur le contexte d'un snapshot donné
    pub fn predict_from_context(&self, snapshot_id: Option<&str>) -> Option<String> {
        let snapshot_id = snapshot_id.unwrap_or("current_snapshot");
        let similar_cases = self.find_similar_cases(snapshot_id);
        let mut decision_count = HashMap::new();

        for case in similar_cases {
            *decision_count.entry(&case.decision).or_insert(0) += 1;
        }

        decision_count
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(decision, _)| decision.clone())
    }

    pub fn record_human_feedback(&mut self, snapshot_id: &str, validated: bool) {
        if let Some(event) = self.nodes.get_mut(snapshot_id) {
            event.human_validated = validated;
        }
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents)?;
        let graph = ron::from_str(&contents)?;
        Ok(graph)
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(path)?;
        to_writer(file, self)?;
        Ok(())
    }

    pub fn save_to_bin(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new().write(true).create(true).open(path)?;
        bincode::serialize_into(file, self)?;
        Ok(())
    }

    pub fn load_from_bin(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = OpenOptions::new().read(true).open(path)?;
        let graph: AiGraph = bincode::deserialize_from(file)?;
        Ok(graph)
    }

    pub fn analyze_memory(&self) -> Vec<String> {
        let mut patterns = Vec::new();
        for (id, event) in &self.nodes {
            if event.human_validated {
                patterns.push(format!("Décision validée : {}", id));
            } else {
                patterns.push(format!("Décision non validée : {}", id));
            }
        }
        patterns
    }

    pub fn save_memory(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(".ai_memory")?;
        // Format humain en RON
        let ron_path = ".ai_memory/memory.ron";
        let ron_file = File::create(ron_path)?;
        to_writer(ron_file, self)?;

        // Format binaire pour l'IA
        let bin_path = ".ai_memory/memory.bin";
        let bin_file = OpenOptions::new().write(true).create(true).open(bin_path)?;
        bincode::serialize_into(bin_file, self)?;
        Ok(())
    }

    pub fn load_memory() -> Option<Self> {
        if let Ok(file) = File::open(".ai_memory/memory.bin") {
            if let Ok(graph) = bincode::deserialize_from(BufReader::new(file)) {
                return Some(graph);
            }
        }

        let file = File::open(".ai_memory/memory.ron").ok()?;
        let mut reader = BufReader::new(file);
        let mut contents = String::new();
        reader.read_to_string(&mut contents).ok()?;
        ron::from_str(&contents).ok()
    }
}
