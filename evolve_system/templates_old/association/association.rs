// Input brut reçu
let raw = "silpou";

// 1. Vectorisation (embeddings bidon ici)
let emb = fake_embed(raw); // ex: [0.24, 0.33]

// 2. Recherche voisins
let voisins = shared_vectors.lock().unwrap().find_similar(&emb, 3);
if voisins.is_empty() || voisins[0].1 < 0.7 {
    // Aucun voisin ou similarité trop faible : NOUVEAU CONCEPT
    shared_graph.lock().unwrap().add_node(GraphNode {
        id: raw.to_string(),
        label: raw.to_string(),
        data: None,
    });
    shared_vectors.lock().unwrap().add_embedding(raw, emb);
    // Demande feedback utilisateur ? Associe à un “parent” si besoin
} else {
    println!("Le plus proche : {:?}", voisins);
    // Ajouter un lien dans le graph vers le plus proche
    let voisin_id = &voisins[0].0;
    shared_graph.lock().unwrap().add_edge(raw, voisin_id, "proche");
}
