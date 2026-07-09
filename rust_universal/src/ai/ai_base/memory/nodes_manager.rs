// Gestionnaire global pour manipuler les nœuds du graphe mémoire de l'IA.

use super::node::MemoryNode;
use super::nodes::MemoryNodes;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub(crate) struct NodesManager { // Visibilité restreinte à `memory`
    nodes: MemoryNodes<MemoryNode>,
    tags: HashMap<usize, Vec<String>>, // Associe des tags aux nœuds par leur ID
}

impl NodesManager {
    pub(crate) fn new() -> Self {
        Self {
            nodes: MemoryNodes::new(),
            tags: HashMap::new(),
        }
    }

    /// Renommé pour cohérence avec LinksManager
    pub(crate) fn insert_node<S: Into<String>>(&mut self, id: usize, data: S) {
        let node = MemoryNode::new(id, data);
        self.nodes.push_node(node);
    }

    /// Renommé pour cohérence avec LinksManager
    pub(crate) fn insert_nodes<I, S>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = (usize, S)>,
        S: Into<String>,
    {
        let nodes = nodes
            .into_iter()
            .map(|(id, data)| MemoryNode::new(id, data));
        self.nodes.push_nodes(nodes);
    }

    /// Ajoute un nœud avec des tags optionnels.
    pub(crate) fn insert_node_with_tags<S: Into<String>>(
        &mut self,
        id: usize,
        data: S,
        tags: Option<Vec<String>>,
    ) {
        let node = MemoryNode::new(id, data);
        self.nodes.push_node(node);

        if let Some(tags) = tags {
            self.tags.insert(id, tags);
        }
    }

    /// Ajoute ou met à jour les tags d'un nœud existant.
    pub(crate) fn add_tags_to_node(&mut self, id: usize, new_tags: Vec<String>) {
        self.tags
            .entry(id)
            .or_insert_with(Vec::new)
            .extend(new_tags);
    }

    /// Récupère les tags associés à un nœud.
    pub(crate) fn get_tags(&self, id: usize) -> Option<&Vec<String>> {
        self.tags.get(&id)
    }

    /// Supprime un tag spécifique d'un nœud.
    pub(crate) fn remove_tag(&mut self, id: usize, tag: &str) -> bool {
        if let Some(tags) = self.tags.get_mut(&id) {
            let initial_len = tags.len();
            tags.retain(|t| t != tag);
            return tags.len() < initial_len;
        }
        false
    }

    /// Supprime tous les tags associés à un nœud.
    pub(crate) fn clear_tags(&mut self, id: usize) {
        self.tags.remove(&id);
    }

    /// Récupère tous les nœuds associés à un tag spécifique.
    pub(crate) fn get_nodes_by_tag(&self, tag: &str) -> Vec<usize> {
        self.tags
            .iter()
            .filter_map(|(id, tags)| {
                if tags.contains(&tag.to_string()) {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Supprime un nœud par id.
    pub(crate) fn remove_node(&mut self, id: usize) -> bool {
        self.nodes.remove_first_node(|n| n.id == id).is_some()
    }

    /// Récupère les données associées à un nœud par id.
    pub(crate) fn get_node_data(&self, id: usize) -> Option<&str> {
        self.nodes.get_node(|n| n.id == id).map(|n| n.data.as_str())
    }

    /// Récupère tous les ids des nœuds.
    pub(crate) fn get_all_ids(&self) -> Vec<usize> {
        self.nodes.all().iter().map(|n| n.id).collect()
    }

    /// Récupère toutes les données des nœuds.
    pub(crate) fn get_all_data(&self) -> Vec<&str> {
        self.nodes.all().iter().map(|n| n.data.as_str()).collect()
    }

    /// Met à jour les données d'un nœud existant par son ID.
    pub(crate) fn update_node_data<S: Into<String>>(&mut self, id: usize, new_data: S) -> bool {
        if let Some(node) = self.nodes.find_first_for_update(|n| n.id == id) {
            node.data = new_data.into();
            true
        } else {
            false
        }
    }

    /// Vérifie si un nœud existe par son ID.
    pub(crate) fn node_exists(&self, id: usize) -> bool {
        self.nodes.find_first(|n| n.id == id).is_some()
    }

    // N'expose jamais MemoryNode ou MemoryNodes à l'extérieur.
}
