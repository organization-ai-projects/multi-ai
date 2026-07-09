//contient l'orchestration des nœuds et liens du graph mémoire de l'ia

use super::links_manager::LinksManager;
use super::nodes_manager::NodesManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, bincode_next::Encode, bincode_next::Decode)]
pub enum MemoryTerm {
    ShortTerm,
    MediumTerm,
    LongTerm,
}

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryGraph {
    pub nodes: NodesManager,
    pub links: LinksManager,
    pub memory: HashMap<String, (String, MemoryTerm)>, // Stockage avec type de mémoire
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self {
            nodes: NodesManager::new(),
            links: LinksManager::new(),
            memory: HashMap::new(),
        }
    }

    /// Ajoute ou met à jour une donnée dans la mémoire avec un type spécifique.
    pub fn store_memory(&mut self, key: impl Into<String>, value: impl Into<String>, term: MemoryTerm) {
        self.memory.insert(key.into(), (value.into(), term));
    }

    /// Récupère une donnée en cherchant en priorité dans la mémoire à court terme, puis moyen, puis long terme.
    pub fn retrieve_memory(&self, key: &str) -> Option<&String> {
        self.memory
            .get(key)
            .map(|(value, _)| value) // Retourne uniquement la valeur
    }

    /// Met à jour la donnée vers la mémoire courte
    pub fn update_to_short_term(&mut self, key: &str) {
        if let Some((value, _)) = self.memory.get_mut(key) {
            *self.memory.get_mut(key).unwrap() = (value.clone(), MemoryTerm::ShortTerm);
        }
    }

    /// Met à jour la donnée vers la mémoire moyenne
    pub fn update_to_medium_term(&mut self, key: &str) {
        if let Some((value, _)) = self.memory.get_mut(key) {
            *self.memory.get_mut(key).unwrap() = (value.clone(), MemoryTerm::MediumTerm);
        }
    }

    /// Met à jour la donnée vers la mémoire longue
    pub fn update_to_long_term(&mut self, key: &str) {
        if let Some((value, _)) = self.memory.get_mut(key) {
            *self.memory.get_mut(key).unwrap() = (value.clone(), MemoryTerm::LongTerm);
        }
    }

    /// Liste toutes les données en mémoire courte
    pub fn list_short_term(&self) -> Vec<&String> {
        self.memory
            .iter()
            .filter_map(|(_, (value, term))| {
                if *term == MemoryTerm::ShortTerm {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Liste toutes les données en mémoire moyenne
    pub fn list_medium_term(&self) -> Vec<&String> {
        self.memory
            .iter()
            .filter_map(|(_, (value, term))| {
                if *term == MemoryTerm::MediumTerm {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Liste toutes les données en mémoire longue
    pub fn list_long_term(&self) -> Vec<&String> {
        self.memory
            .iter()
            .filter_map(|(_, (value, term))| {
                if *term == MemoryTerm::LongTerm {
                    Some(value)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Ajoute un nœud à partir de ses données, sans dépendance à MemoryNode.
    pub fn add_node<S: Into<String>>(&mut self, id: usize, data: S) {
        self.nodes.insert_node(id, data);
    }

    /// Ajoute plusieurs nœuds à partir d'itérables de (id, data), sans dépendance à MemoryNode.
    pub fn add_nodes<I, S>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = (usize, S)>,
        S: Into<String>,
    {
        self.nodes.insert_nodes(nodes);
    }

    pub fn add_link(&mut self, from: usize, to: usize, weight: f32) {
        self.links.insert_link(from, to, weight);
    }

    pub fn add_links<I>(&mut self, links: I)
    where
        I: IntoIterator<Item = (usize, usize, f32)>,
    {
        self.links.insert_links(links);
    }

    /// Ajoute un lien bidirectionnel entre deux nœuds.
    pub fn add_bidirectional_link(&mut self, from: usize, to: usize, weight: f32) {
        self.links.insert_bidirectional_link(from, to, weight);
    }

    /// Vérifie si un lien existe entre deux nœuds.
    pub fn link_exists(&self, from: usize, to: usize) -> bool {
        self.links.get_link_weight(from, to).is_some()
    }

    /// Met à jour les données d'un nœud existant.
    pub fn update_node_data<S: Into<String>>(&mut self, id: usize, new_data: S) -> bool {
        self.nodes.update_node_data(id, new_data)
    }

    /// Vérifie si un nœud existe par son ID.
    pub fn node_exists(&self, id: usize) -> bool {
        self.nodes.node_exists(id)
    }

    /// Vérifie que tous les liens référencent des nœuds existants dans le graphe.
    pub fn is_valid(&self) -> bool {
        let node_ids: std::collections::HashSet<_> = self.nodes.get_all_ids().into_iter().collect();
        self.links
            .all()
            .iter()
            .all(|link| node_ids.contains(&link.from) && node_ids.contains(&link.to))
    }

    pub fn store_short_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.store_memory(key, value, MemoryTerm::ShortTerm);
    }

    pub fn store_medium_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.store_memory(key, value, MemoryTerm::MediumTerm);
    }

    pub fn store_long_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.store_memory(key, value, MemoryTerm::LongTerm);
    }

    pub fn retrieve_short_term(&self, key: &str) -> Option<&String> {
        self.retrieve_memory(key)
    }

    /// Déplace toutes les données de la mémoire courte vers la mémoire moyenne
    pub fn promote_short_term_to_medium(&mut self) {
        for (key, (value, term)) in self.memory.iter_mut() {
            if *term == MemoryTerm::ShortTerm {
                *term = MemoryTerm::MediumTerm;
            }
        }
    }

    pub fn retrieve_archived_short_term(&self, key: &str) -> Option<&String> {
        self.retrieve_memory(key)
    }
}
