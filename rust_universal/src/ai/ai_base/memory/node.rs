use serde::{Deserialize, Serialize};

// Définit la structure et les méthodes d'un nœud du graphe mémoire de l'IA.

#[derive(Debug, Clone, Default, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub(crate) struct MemoryNode {
    pub id: usize,
    pub data: String,
}

impl MemoryNode {
    pub(crate) fn new(id: usize, data: impl Into<String>) -> Self {
        Self {
            id,
            data: data.into(),
        }
    }
}
