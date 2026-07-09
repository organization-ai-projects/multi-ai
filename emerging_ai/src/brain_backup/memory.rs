use serde::{Serialize, Deserialize};
use uuid::Uuid;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::Directed;
use std::collections::HashMap;
use crate::capacities::mutate_ast::Strategy;

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum NodeType {
    SourceSnippet { code: String, file: String },
    MutatedSnippet {
        code: String,
        parent: Uuid,
        parents: Vec<Uuid>, // Lignée complète
        mutation_type: String,
        mutation_reason: String, // Raison de la mutation
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryNode {
    pub id: Uuid,
    pub kind: NodeType,
    pub successes: u32,
    pub failures: u32,
    pub last_compilation_error: Option<String>,
    pub error_history: Vec<String>, // Historique des erreurs
    pub timestamp: Option<u64>, // Timestamp pour suivre l'évolution
    pub avg_compilation_time: Option<f64>, // Temps moyen de compilation
    pub mutation_count: u32, // Nombre de mutations générées
    pub cluster_id: Option<u32>, // Identifiant du cluster ou de la communauté
    pub strategy_weights: HashMap<Strategy, f64>, // Poids des stratégies
}

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryEdge {
    pub description: String,
    pub weight: u32, // Nombre d'expositions
    pub strength: f32, // Force positive ou négative
    pub relation_type: String, // Type de relation (mutation, feedback, etc.)
    pub is_dependency: bool,  // Indique si c'est une relation de dépendance
}

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct FileExtensionInfo {
    pub extension: String,
    pub occurrences: u32,
    pub associated_with_rust: bool,
}

#[derive(Serialize, Deserialize, Debug, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryGraph {
    // Stockage graphique (pas direct Petgraph pour sérialisation facile)
    pub nodes: HashMap<Uuid, MemoryNode>,
    pub edges: Vec<(Uuid, Uuid, MemoryEdge)>, // (from, to, info)
    pub file_extensions: HashMap<String, FileExtensionInfo>, // Enregistre les extensions de fichiers
}

impl MemoryGraph {
    pub fn new() -> Self {
        MemoryGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
            file_extensions: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: MemoryNode) {
        self.nodes.insert(node.id, node);
    }

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
        self.edges.push((from, to, edge.clone()));

        // Ajout de l'arête inverse pour rendre le graphe bidirectionnel
        let reverse_edge = MemoryEdge {
            description: format!("inverse_{}", description),
            weight,
            strength: -strength, // Force inverse
            relation_type: format!("inverse_{}", relation_type),
            is_dependency,
        };
        self.edges.push((to, from, reverse_edge));
    }

    pub fn update_edge(&mut self, from: Uuid, to: Uuid, delta_weight: u32, delta_strength: f32) {
        for (src, dst, edge) in &mut self.edges {
            if *src == from && *dst == to {
                edge.weight += delta_weight;
                edge.strength += delta_strength;
            }
        }
    }

    pub fn record_extension(&mut self, extension: &str, associated_with_rust: bool) {
        let entry = self.file_extensions.entry(extension.to_string()).or_insert(FileExtensionInfo {
            extension: extension.to_string(),
            occurrences: 0,
            associated_with_rust,
        });
        entry.occurrences += 1;
        if associated_with_rust {
            entry.associated_with_rust = true;
        }
    }

    pub fn add_mutated_node(
        &mut self,
        parent_id: Uuid,
        code: String,
        mutation_type: &str,
        mutation_reason: &str,
        success: bool,
        error_message: Option<String>,
    ) {
        let parent_node = self.nodes.get(&parent_id).expect("Parent node not found");
        let new_id = Uuid::new_v7();
        let mut_node = MemoryNode {
            id: new_id,
            kind: NodeType::MutatedSnippet {
                code,
                parent: parent_id,
                parents: match &parent_node.kind {
                    NodeType::MutatedSnippet { parents, .. } => {
                        let mut lineage = parents.clone();
                        lineage.push(parent_id);
                        lineage
                    }
                    _ => vec![parent_id],
                },
                mutation_type: mutation_type.to_string(),
                mutation_reason: mutation_reason.to_string(),
            },
            successes: if success { 1 } else { 0 },
            failures: if success { 0 } else { 1 },
            last_compilation_error: error_message.clone(),
            error_history: if let Some(err) = error_message {
                vec![err]
            } else {
                vec![]
            },
            timestamp: None,
            avg_compilation_time: None,
            mutation_count: 0,
            cluster_id: parent_node.cluster_id,
            strategy_weights: parent_node.strategy_weights.clone(), // Héritage des poids de stratégie
        };
        self.nodes.insert(new_id, mut_node);

        let strength = if success { 1.0 } else { -1.0 };
        self.add_edge(parent_id, new_id, mutation_type, 1, strength, "mutation", false);
    }

    pub fn add_snippet(&mut self, code: String, path: String) {
        let id = Uuid::new_v7();
        let node = MemoryNode {
            id,
            kind: NodeType::SourceSnippet { code, file: path },
            successes: 0,
            failures: 0,
            ..Default::default()
        };
        self.nodes.insert(id, node);
    }

    pub fn pick_random_snippet(&self) -> Option<(String, Uuid)> {
        let mut rng = rand::rng();
        self.nodes.values().filter_map(|node| {
            if let NodeType::SourceSnippet { code, .. } = &node.kind {
                Some((code.clone(), node.id))
            } else {
                None
            }
        }).choose(&mut rng)
    }

    pub fn feedback(
        &mut self,
        id: Uuid,
        mutated_code: &str,
        success: bool,
        mutation_type: &str,
        mutation_reason: &str,
        error_message: Option<String>,
    ) {
        if let Some(node) = self.nodes.get_mut(&id) {
            if success {
                node.successes += 1;
            } else {
                node.failures += 1;
                if let Some(err) = &error_message {
                    node.error_history.push(err.clone());
                }
            }
            node.mutation_count += 1;
        }

        self.add_mutated_node(
            id,
            mutated_code.to_string(),
            mutation_type,
            mutation_reason,
            success,
            error_message,
        );
    }

    pub fn load_snippets_by_limit(&self, limit: usize) -> Vec<(String, Uuid)> {
        self.nodes.values()
            .filter_map(|node| {
                if let NodeType::SourceSnippet { code, .. } = &node.kind {
                    Some((code.clone(), node.id))
                } else {
                    None
                }
            })
            .take(limit)
            .collect()
    }

    pub fn load_snippets_paginated(&self, page: usize, page_size: usize) -> Vec<(String, Uuid)> {
        self.nodes.values()
            .filter_map(|node| {
                if let NodeType::SourceSnippet { code, .. } = &node.kind {
                    Some((code.clone(), node.id))
                } else {
                    None
                }
            })
            .skip(page * page_size)
            .take(page_size)
            .collect()
    }

    pub fn index_snippets_by_success(&self) -> Vec<(Uuid, u32)> {
        self.nodes.iter()
            .filter_map(|(id, node)| {
                if let NodeType::SourceSnippet { .. } = node.kind {
                    Some((*id, node.successes))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl MemoryNode {
    pub fn adjust_strategy_weight(&mut self, strategy: &Strategy, success: bool) {
        let weight = self.strategy_weights.entry(strategy.clone()).or_insert(1.0);
        if success {
            *weight += 0.2; // Augmente le poids en cas de succès
        } else {
            *weight *= 0.9; // Réduit le poids en cas d'échec
        }
    }
}
