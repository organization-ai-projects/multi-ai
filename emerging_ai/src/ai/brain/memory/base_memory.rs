use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient la mémoire de l'IA de base qui contiendra uniquement
//les uuid des nœuds et des liens
//utilise ce qui a déjà été défini dans memory/nodes.rs et memory/links.rs

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BaseMemory {
    pub node_uuids: Vec<Uuid>,       // UUIDs des nœuds
    pub link_uuids: Vec<Uuid>,       // UUIDs des liens
    pub timestamp: u64,              // Timestamp de la dernière mise à jour
}

impl BaseMemory {
    pub fn new(node_uuids: Vec<Uuid>, link_uuids: Vec<Uuid>, timestamp: u64) -> Self {
        Self {
            node_uuids,
            link_uuids,
            timestamp,
        }
    }

    pub fn add_node_uuid(&mut self, node_uuid: Uuid) {
        self.node_uuids.push(node_uuid);
    }

    pub fn add_link_uuid(&mut self, link_uuid: Uuid) {
        self.link_uuids.push(link_uuid);
    }
}