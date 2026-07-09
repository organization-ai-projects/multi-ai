/// Ce fichier implémente un graphe de connaissances (`KnowledgeGraph`).
/// Rôle : Stocker et gérer des relations entre des mots ou concepts sous forme de nœuds et d'arêtes.
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: HashSet<String>,
    pub edges: HashMap<String, HashSet<String>>, // mot -> ensemble de voisins
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_words(&mut self, url: &str, words: &[String]) {
        for word in words {
            self.nodes.insert(word.clone());
            self.edges
                .entry(url.to_string())
                .or_default()
                .insert(word.clone());
        }
    }
}
