use std::collections::{HashMap, HashSet};

// Un nœud du graph
#[derive(Debug, Clone)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub data: Option<String>,
    pub neighbors: HashSet<String>, // Set d'IDs liés
}

// Le graph global
pub struct Graph {
    pub nodes: HashMap<String, GraphNode>,
}
impl Graph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.insert(node.id.clone(), node);
    }
    pub fn add_edge(&mut self, a: &str, b: &str) {
        if let Some(na) = self.nodes.get_mut(a) {
            na.neighbors.insert(b.to_string());
        }
        if let Some(nb) = self.nodes.get_mut(b) {
            nb.neighbors.insert(a.to_string());
        }
    }
}

// Fake vector index (peut être un HashMap ou un vrai ANN plus tard)
pub struct VectorIndex {
    pub embeddings: HashMap<String, Vec<f32>>, // id -> vecteur
}
impl VectorIndex {
    pub fn new() -> Self { Self { embeddings: HashMap::new() } }
    pub fn add(&mut self, id: &str, v: Vec<f32>) { self.embeddings.insert(id.to_string(), v); }
    // Trouver le plus proche (simili-cosine)
    pub fn find_similar(&self, v: &[f32]) -> Option<(String, f32)> {
        self.embeddings
            .iter()
            .map(|(id, emb)| (id.clone(), cosine_sim(emb, v)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .filter(|x| x.1 > 0.7)
    }
}
// Cosine bidon
fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 { 0.0 } else { dot / (norm_a * norm_b) }
}

// Mini encodeur tout bête
fn encode(text: &str) -> Vec<f32> {
    let mut v = vec![0.0; 8];
    for (i, c) in text.chars().enumerate() {
        v[i % 8] += (c as u32 % 31) as f32;
    }
    v
}
