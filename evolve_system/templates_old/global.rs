// Simplicité : tu spécialises ensuite par IA, graph, etc.

pub struct MyLocalMemory {
    store: std::collections::HashMap<String, String>,
}

impl LocalMemory for MyLocalMemory {
    fn get(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }
    fn set(&mut self, key: &str, value: String) {
        self.store.insert(key.to_string(), value);
    }
    fn all(&self) -> Vec<(String, String)> {
        self.store
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
}

// Structures de graphe (à adapter !)
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub data: Option<String>,
}

pub struct GraphEdge {
    pub from: String,
    pub to: String,
    pub label: String,
}

pub struct MyKnowledgeGraph {
    nodes: std::collections::HashMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
}

impl KnowledgeGraph for MyKnowledgeGraph {
    // ... (implémente les méthodes)
    // cf. plus haut
}

// Index vectoriel "débile"
pub struct SimpleVectorIndex {
    pub vectors: std::collections::HashMap<String, Vec<f32>>,
}

impl VectorIndex for SimpleVectorIndex {
    fn add_embedding(&mut self, id: &str, vector: Vec<f32>) {
        self.vectors.insert(id.to_string(), vector);
    }
    fn find_similar(&self, vector: &[f32], top_k: usize) -> Vec<(String, f32)> {
        // Retourne les top_k plus proches (brute-force, cosine sim)
        let mut sims = self
            .vectors
            .iter()
            .map(|(k, v)| {
                let sim = cosine_similarity(vector, v);
                (k.clone(), sim)
            })
            .collect::<Vec<_>>();
        sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sims.into_iter().take(top_k).collect()
    }
    fn get_embedding(&self, id: &str) -> Option<Vec<f32>> {
        self.vectors.get(id).cloned()
    }
}

// Cosine sim simple
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a * norm_b)
    }
}
