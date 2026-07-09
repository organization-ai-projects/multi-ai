use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub label: String,
}

#[derive(Debug, Serialize, Deserialize, Default, bincode_next::Encode, bincode_next::Decode)]
pub struct GraphMemory {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

impl GraphMemory {
    pub fn save_ron(&self, path: &str) -> Result<(), String> {
        let ron = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Erreur RON: {}", e))?;
        fs::write(path, ron).map_err(|e| format!("Erreur écriture: {}", e))
    }

    pub fn save_bin(&self, path: &str) -> Result<(), String> {
        let bin = bincode_next::encode_to_vec(self, bincode_next::config::standard()).map_err(|e| format!("Erreur bincode: {}", e))?;
        fs::write(path, bin).map_err(|e| format!("Erreur écriture: {}", e))
    }

    pub fn load_ron(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path).map_err(|e| format!("Erreur lecture: {}", e))?;
        ron::from_str(&content).map_err(|e| format!("Erreur parsing RON: {}", e))
    }

    pub fn load_bin(path: &str) -> Result<Self, String> {
        let content = fs::read(path).map_err(|e| format!("Erreur lecture: {}", e))?;
        bincode_next::decode_from_slice(&content, bincode_next::config::standard()).map(|(v, _)| v).map_err(|e| format!("Erreur parsing bincode: {}", e))
    }

    pub fn add_or_update_node(&mut self, node: Node) {
        if let Some(existing) = self.nodes.iter_mut().find(|n| n.id == node.id) {
            *existing = node;
        } else {
            self.nodes.push(node);
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.push(edge);
    }
}
