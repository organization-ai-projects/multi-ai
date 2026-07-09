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
}
