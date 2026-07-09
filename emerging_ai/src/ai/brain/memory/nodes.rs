use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient les noeuds du graphe mémoire de l'IA.

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryNode {
    pub uuid: Uuid,           // Identifiant unique du nœud
    pub label: String,        // Étiquette ou nom du nœud
    pub data: Option<String>, // Données associées au nœud
    pub timestamp: u64,       // Timestamp de création ou de mise à jour
}

impl MemoryNode {
    pub fn new(uuid: Uuid, label: String, data: Option<String>, timestamp: u64) -> Self {
        Self { uuid, label, data, timestamp }
    }

    pub fn update_label(&mut self, new_label: String) {
        self.label = new_label;
    }

    pub fn update_data(&mut self, new_data: Option<String>) {
        self.data = new_data;
    }
}