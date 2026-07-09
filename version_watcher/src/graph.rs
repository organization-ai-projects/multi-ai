use crate::version::VersionSnapshot; // Ajout de l'import manquant
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum Impact {
    Patch,
    Minor,
    Major,
}

#[derive(Serialize, Deserialize, Debug, bincode_next::Encode, bincode_next::Decode)]
pub struct VersionGraph {
    pub nodes: HashMap<String, GraphNode>,
    pub edges: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug, bincode_next::Encode, bincode_next::Decode)]
pub struct GraphNode {
    pub id: String,
    pub impact: Impact,
    pub timestamp: i64,
    pub hash: String,
}

impl VersionGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: GraphNode) {
        if !self.nodes.contains_key(&node.id) {
            if let Some(last) = self.latest_id() {
                self.edges.push((last.clone(), node.id.clone()));
            }
            self.nodes.insert(node.id.clone(), node);
        }
    }

    pub fn latest_id(&self) -> Option<String> {
        self.nodes.keys().max().cloned()
    }

    pub fn calculate_impact(&self, old_hash: &str, new_hash: &str) -> Impact {
        estimate_impact_from_diff(old_hash, new_hash)
    }

    pub fn analyze_changes(&self, from_id: &str, to_id: &str) -> Option<String> {
        let from_node = self.nodes.get(from_id)?;
        let to_node = self.nodes.get(to_id)?;
        
        let diff_count = count_diff_files(&from_node.hash, &to_node.hash);
        let impact = estimate_impact_from_diff(&from_node.hash, &to_node.hash);
        
        Some(format!(
            "Analyse des changements:\n- Fichiers modifiés: {}\n- Impact estimé: {:?}",
            diff_count,
            impact
        ))
    }
}

pub fn print_graph_log() {
    let graph = load_graph();
    for (id, node) in &graph.nodes {
        println!(
            "ID: {}, Impact: {:?}, Timestamp: {}",
            id, node.impact, node.timestamp
        );
    }
}

pub fn print_latest_snapshot() {
    let graph = load_graph();
    if let Some(latest_id) = graph.latest_id() {
        println!("Dernier snapshot : {}", latest_id);
    } else {
        println!("Aucun snapshot trouvé.");
    }
}

pub fn revert_to_snapshot(id: &str) {
    let graph = load_graph();
    if let Some(node) = graph.nodes.get(id) {
        restore_snapshot(node);
        println!("✅ Snapshot '{}' restauré.", id);
    } else {
        eprintln!("❌ Snapshot '{}' introuvable.", id);
    }
}

pub fn load_graph() -> VersionGraph {
    let graph_path = ".graphver/graph.ron";
    if Path::new(graph_path).exists() {
        ron::from_str(&std::fs::read_to_string(graph_path).unwrap())
            .unwrap_or_else(|_| VersionGraph::new())
    } else {
        VersionGraph::new()
    }
}

fn restore_snapshot(node: &GraphNode) {
    let bin_path = format!(".graphver/snapshots/{}.bin", node.id);
    if let Ok(snapshot_data) = std::fs::read(&bin_path) {
        if let Ok(snapshot) = bincode_next::decode_from_slice::<VersionSnapshot, _>(&snapshot_data, bincode_next::config::standard()).map(|(v, _)| v) {
            // Restaurer les fichiers (à implémenter)
            println!("🔄 Restauration des fichiers pour le snapshot : {:?}", snapshot);
        }
    }
}

fn estimate_impact_from_diff(old_hash: &str, new_hash: &str) -> Impact {
    if old_hash == new_hash {
        Impact::Patch
    } else {
        let diff_count = count_diff_files(old_hash, new_hash);
        if diff_count < 3 {
            Impact::Minor
        } else {
            Impact::Major
        }
    }
}

fn count_diff_files(old_hash: &str, new_hash: &str) -> usize {
    // Simule une comparaison entre deux états (à améliorer avec un vrai diff)
    if old_hash != new_hash {
        5 // Exemple : retourne un nombre fixe pour l'instant
    } else {
        0
    }
}

pub fn merge_graphs(graphs: Vec<VersionGraph>) -> VersionGraph {
    let mut merged_graph = VersionGraph::new();
    for graph in graphs {
        for (id, node) in graph.nodes {
            if !merged_graph.nodes.contains_key(&id) {
                merged_graph.add_node(node);
            }
        }
        merged_graph.edges.extend(graph.edges);
    }
    merged_graph
}
