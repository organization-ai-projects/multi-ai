// Accès à la mémoire locale (chaque IA a la sienne)
pub trait LocalMemory {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, value: String);
    fn all(&self) -> Vec<(String, String)>;
}

// Accès au graphe universel (mutualisé)
pub trait KnowledgeGraph {
    fn get_node(&self, id: &str) -> Option<GraphNode>;
    fn add_node(&mut self, node: GraphNode);
    fn add_edge(&mut self, from: &str, to: &str, label: &str);
    fn find_links(&self, query: &str) -> Vec<GraphEdge>;
    // + tout ce que tu veux (corriger, supprimer, annoter, etc.)
}

// Accès à l’index vectoriel partagé
pub trait VectorIndex {
    fn add_embedding(&mut self, id: &str, vector: Vec<f32>);
    fn find_similar(&self, vector: &[f32], top_k: usize) -> Vec<(String, f32)>;
    fn get_embedding(&self, id: &str) -> Option<Vec<f32>>;
}
