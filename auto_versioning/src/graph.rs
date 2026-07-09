use petgraph::{Graph, Directed};
use petgraph::graph::NodeIndex;
use petgraph::algo::has_path_connecting;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::versioning::types::ChangeSet;
use std::sync::{Arc, Mutex};

pub type DiGraph = Graph<ChangeSet, (), Directed>;

#[derive(Debug, Default)]
pub struct DependencyGraph {
    graph: DiGraph,
    node_indices: HashMap<String, NodeIndex>,
}

impl Serialize for DependencyGraph {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("DependencyGraph", 1)?;
        state.serialize_field("nodes", &self.graph.node_weights().collect::<Vec<_>>())?;
        state.end()
    }
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_change(&mut self, change: ChangeSet) {
        let node_idx = self.get_or_create_node(&change);
        for target in self.graph.node_indices() {
            let target_node = &self.graph[target];
            if change.path.contains(&target_node.path) {
                self.graph.add_edge(node_idx, target, ());
            }
        }
    }

    pub fn analyze_impacts(&self) -> Vec<(String, String)> {
        let mut impacts = Vec::new();
        for source in self.graph.node_indices() {
            for target in self.graph.node_indices() {
                if has_path_connecting(&self.graph, source, target, None) {
                    let source_node = &self.graph[source];
                    let target_node = &self.graph[target];
                    impacts.push((target_node.path.clone(), source_node.version.clone()));
                }
            }
        }
        impacts
    }

    fn get_or_create_node(&mut self, node: &ChangeSet) -> NodeIndex {
        if let Some(&idx) = self.node_indices.get(&node.path) {
            idx
        } else {
            let idx = self.graph.add_node(node.clone());
            self.node_indices.insert(node.path.clone(), idx);
            idx
        }
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = &ChangeSet> {
        self.graph.node_weights()
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "nodes": self.graph.node_weights().collect::<Vec<_>>(),
            "edges": self.graph.edge_indices().map(|e| {
                let (s, t) = self.graph.edge_endpoints(e).unwrap();
                (self.graph[s].path.clone(), self.graph[t].path.clone())
            }).collect::<Vec<_>>()
        })
    }

    pub fn save(&self, _name: &str) -> std::io::Result<()> {
        // Implémentation de la sauvegarde
        Ok(())
    }
}

pub type SharedGraph = Arc<Mutex<DependencyGraph>>;
