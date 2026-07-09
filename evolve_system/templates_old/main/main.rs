use std::sync::{Arc, Mutex};

fn main() {
    let shared_graph = Arc::new(Mutex::new(MyKnowledgeGraph {
        nodes: Default::default(),
        edges: vec![],
    }));
    let shared_vectors = Arc::new(Mutex::new(SimpleVectorIndex {
        vectors: Default::default(),
    }));

    // Chaque IA a sa propre mémoire mais partage le “cerveau universel”
    let mut ia1 = MyAI::new(shared_graph.clone(), shared_vectors.clone());
    let mut ia2 = MyAI::new(shared_graph.clone(), shared_vectors.clone());

    ia1.learn_from_input("salut", "forme de salutation", vec![0.1, 0.9]);
    ia2.learn_from_input("bonjour", "autre salutation", vec![0.12, 0.85]);

    let proches = ia1.query_similar(&[0.11, 0.87]);
    println!("Plus proches de [0.11, 0.87] : {:?}", proches);

    // Chacun peut explorer le graphe et enrichir la base de connaissances
}
