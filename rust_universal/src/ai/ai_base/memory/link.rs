use serde::{Deserialize, Serialize};

// Définit la structure et les méthodes d'un lien du graphe mémoire de l'IA.

#[derive(Debug, Clone, Default, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub(crate) struct MemoryLink {
    pub from: usize,
    pub to: usize,
    pub weight: f32,
}

impl MemoryLink {
    pub(crate) fn new(from: usize, to: usize, weight: f32) -> Self {
        Self { from, to, weight }
    }
}
