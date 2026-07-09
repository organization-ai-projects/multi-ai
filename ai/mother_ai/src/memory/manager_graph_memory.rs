use std::collections::HashMap;
use super::nodes_manager::NodesManager;
use super::links_manager::LinksManager;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct GraphMemoryManager {
    nodes_manager: NodesManager,
    links_manager: LinksManager,
}

impl GraphMemoryManager {
    pub fn new() -> Self {
        Self {
            nodes_manager: NodesManager::new(),
            links_manager: LinksManager::new(),
        }
    }

    // =============== Délégation à NodesManager ===============
    
    // Crée et ajoute un nouveau nœud avec ID et label
    pub fn create_and_add_node(&mut self, id: &str, label: &str) -> String {
        self.nodes_manager.create_and_add_node(id, label)
    }
    
    // Commence la construction d'un nouveau nœud
    pub fn create_node_begin(&mut self, id: &str, label: &str) -> String {
        self.nodes_manager.create_node_begin(id, label)
    }
    
    // Ajoute un attribut au nœud en construction
    pub fn create_node_add_attribute(&mut self, id: &str, key: &str, value: &str) -> bool {
        self.nodes_manager.create_node_add_attribute(id, key, value)
    }
    
    // Finalise la création du nœud
    pub fn create_node_finish(&mut self, id: &str) -> Option<String> {
        self.nodes_manager.create_node_finish(id)
    }

    // Ajoute un tag à un nœud existant
    pub fn add_tag_to_node(&mut self, node_id: &str, tag: &str) -> bool {
        self.nodes_manager.add_tag_to_node(node_id, tag)
    }

    // Vérifie si un nœud a un tag spécifique
    pub fn node_has_tag(&self, node_id: &str, tag: &str) -> bool {
        self.nodes_manager.node_has_tag(node_id, tag)
    }

    // Récupère un attribut d'un nœud
    pub fn get_node_attribute(&self, node_id: &str, attribute: &str) -> Option<String> {
        self.nodes_manager.get_node_attribute(node_id, attribute)
    }
    
    // Récupère le label d'un nœud
    pub fn get_node_label(&self, id: &str) -> Option<String> {
        self.nodes_manager.get_node_label(id)
    }
    
    // Vérifie si un nœud existe
    pub fn node_exists(&self, id: &str) -> bool {
        self.nodes_manager.node_exists(id)
    }

    // Récupère tous les IDs de nœuds
    pub fn get_all_node_ids(&self) -> Vec<String> {
        self.nodes_manager.get_all_node_ids()
    }
    
    // =============== Gestion des liens ===============
    
    // Crée et ajoute un lien entre deux noeuds
    pub fn create_and_add_link(&mut self, source: &str, target: &str, label: Option<String>, weight: Option<f32>) -> bool {
        // Vérifie d'abord que les deux noeuds existent
        if self.node_exists(source) && self.node_exists(target) {
            let link = self.links_manager.create_link(source, target, label, weight);
            self.links_manager.add_link(link);
            true
        } else {
            false
        }
    }
    
    // Obtient tous les liens pour un noeud donné
    pub fn get_links_for_node(&self, node_id: &str) -> Vec<(String, String, Option<String>, Option<f32>)> {
        // Si le noeud n'existe pas, on retourne une liste vide
        if !self.node_exists(node_id) {
            return Vec::new();
        }
        
        // On demande au gestionnaire de liens de nous donner les informations sous forme de tuples
        self.links_manager.get_links_for_node(node_id)
            .iter()
            .map(|link_info| {
                (
                    link_info.source.clone(), 
                    link_info.target.clone(),
                    link_info.label.clone(),
                    link_info.weight
                )
            })
            .collect()
    }

    /// Récupère tous les liens sous forme brute (source, target, label, weight)
    pub fn get_links_for_all_nodes(&self) -> Vec<(String, String, Option<String>, Option<f32>)> {
        self.links_manager.get_all_links()
    }
}
