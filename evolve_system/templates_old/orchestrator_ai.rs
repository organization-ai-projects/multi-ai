pub struct MyAI {
    local_memory: MyLocalMemory,
    graph: std::sync::Arc<std::sync::Mutex<MyKnowledgeGraph>>,
    vectors: std::sync::Arc<std::sync::Mutex<SimpleVectorIndex>>,
    // Ajoute d’autres modules selon besoin
}

impl MyAI {
    pub fn new(
        graph: Arc<Mutex<MyKnowledgeGraph>>,
        vectors: Arc<Mutex<SimpleVectorIndex>>,
    ) -> Self {
        Self {
            local_memory: MyLocalMemory {
                store: Default::default(),
            },
            graph,
            vectors,
        }
    }

    // Exemples de requêtes
    pub fn learn_from_input(&mut self, key: &str, value: &str, embedding: Vec<f32>) {
        self.local_memory.set(key, value.to_string());
        // Ajoute un nœud dans le graphe
        self.graph.lock().unwrap().add_node(GraphNode {
            id: key.to_string(),
            label: value.to_string(),
            data: None,
        });
        // Ajoute embedding
        self.vectors.lock().unwrap().add_embedding(key, embedding);
    }

    pub fn query_similar(&self, vector: &[f32]) -> Vec<(String, f32)> {
        self.vectors.lock().unwrap().find_similar(vector, 5)
    }

    pub fn graph_explore(&self, start: &str) -> Vec<String> {
        self.graph
            .lock()
            .unwrap()
            .find_links(start)
            .into_iter()
            .map(|e| e.to.clone())
            .collect()
    }
}
