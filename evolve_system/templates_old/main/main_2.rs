fn main() {
    let mut loop_ai = LearningLoop::new();

    let dataset = vec![
        "salut",
        "bonjour",
        "coucou",
        "hello",
        "maison",
        "appartement",
        "immeuble",
        "home",
        "chien",
        "chat",
        "oiseau",
        "dog",
        "cat",
        "silpou", // mot inventé
    ];

    for mot in &dataset {
        loop_ai.learn(mot);
    }

    println!("Graph final :");
    for (id, node) in &loop_ai.graph.nodes {
        println!("{id} : voisins -> {:?}", node.neighbors);
    }
}
