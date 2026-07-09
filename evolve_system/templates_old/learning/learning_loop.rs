pub struct LearningLoop {
    pub graph: Graph,
    pub vectors: VectorIndex,
}

impl LearningLoop {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            vectors: VectorIndex::new(),
        }
    }

    pub fn learn(&mut self, input: &str) {
        let v = encode(input);
        if let Some((best_id, score)) = self.vectors.find_similar(&v) {
            println!("Reconnu : {input} ~> {best_id} (score {score:.2})");
            self.graph.add_edge(&input, &best_id);
        } else {
            println!("Nouveau concept : {input}");
            self.graph.add_node(GraphNode {
                id: input.to_string(),
                label: input.to_string(),
                data: None,
                neighbors: HashSet::new(),
            });
            self.vectors.add(input, v);
        }
    }

    pub fn save(&self, path: &str) {
        let data = (
            ron::to_string(&self.graph).unwrap(),
            ron::to_string(&self.vectors).unwrap(),
        );
        std::fs::write(format!("{path}_graph.ron"), data.0).unwrap();
        std::fs::write(format!("{path}_vecs.ron"), data.1).unwrap();
    }

    pub fn load(path: &str) -> Option<Self> {
        let g = std::fs::read_to_string(format!("{path}_graph.ron")).ok()?;
        let v = std::fs::read_to_string(format!("{path}_vecs.ron")).ok()?;
        let graph = ron::from_str(&g).ok()?;
        let vectors = ron::from_str(&v).ok()?;
        Some(Self { graph, vectors })
    }
}
