use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient les liens du graphe mémoire de l'IA.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryLink {
    pub uuid: Uuid,           // Identifiant unique du lien
    pub source: Uuid,         // UUID du nœud source
    pub target: Uuid,         // UUID du nœud cible
    pub weight: f32,          // Poids ou importance du lien
    pub timestamp: u64,       // Timestamp de création ou de mise à jour
}

impl MemoryLink {
    pub fn new(uuid: Uuid, source: Uuid, target: Uuid, weight: f32, timestamp: u64) -> Self {
        Self { uuid, source, target, weight, timestamp }
    }

    pub fn update_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}