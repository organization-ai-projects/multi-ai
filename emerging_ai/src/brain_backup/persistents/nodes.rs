use crate::brain::memory::{MemoryGraph, MemoryNode, MemoryEdge};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Write, Read};
use bincode;
use ron;
use uuid::Uuid;

pub struct NodeManager {
    pub graph: MemoryGraph, // Instance de MemoryGraph directement accessible
}

impl NodeManager {
    pub fn new() -> Self {
        NodeManager {
            graph: MemoryGraph::new(),
        }
    }

    /// Sauvegarde le graphe en format binaire
    pub fn save_bin(&self, path: &str) {
        let file = File::create(path).expect("Could not create bin file");
        bincode_next::encode_into_std_write(&self.graph, &mut file, bincode_next::config::standard()).expect("Could not serialize bin");
    }

    /// Charge le graphe depuis un fichier binaire
    pub fn load_bin(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open bin file");
        self.graph = bincode_next::decode_from_std_read(&mut file, bincode_next::config::standard()).expect("Could not deserialize bin");
    }

    /// Sauvegarde le graphe en format RON
    pub fn save_ron(&self, path: &str) {
        let file = File::create(path).expect("Could not create RON file");
        ron::ser::to_writer(file, &self.graph).expect("Could not serialize RON");
    }

    /// Charge le graphe depuis un fichier RON
    pub fn load_ron(&mut self, path: &str) {
        let file = File::open(path).expect("Could not open RON file");
        self.graph = ron::de::from_reader(file).expect("Could not deserialize RON");
    }

    /// Ajoute un nœud au graphe
    pub fn add_node(&mut self, node: MemoryNode) {
        self.graph.nodes.insert(node.id, node);
    }

    /// Ajoute une arête au graphe
    pub fn add_edge(
        &mut self,
        from: Uuid,
        to: Uuid,
        description: &str,
        weight: u32,
        strength: f32,
        relation_type: &str,
        is_dependency: bool,
    ) {
        let edge = MemoryEdge {
            description: description.into(),
            weight,
            strength,
            relation_type: relation_type.into(),
            is_dependency,
        };
        self.graph.edges.push((from, to, edge.clone()));

        // Ajout de l'arête inverse pour rendre le graphe bidirectionnel
        let reverse_edge = MemoryEdge {
            description: format!("inverse_{}", description),
            weight,
            strength: -strength, // Force inverse
            relation_type: format!("inverse_{}", relation_type),
            is_dependency,
        };
        self.graph.edges.push((to, from, reverse_edge));
    }
}
