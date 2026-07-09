use std::collections::HashMap;
use super::node::{Node, NodeBuilder};

pub struct NodesManager {
    nodes: HashMap<String, Node>,
    nodes_in_construction: HashMap<String, HashMap<String, String>>,
}

impl NodesManager {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            nodes_in_construction: HashMap::new(),
        }
    }

    // Crée et ajoute un nouveau nœud avec ID et label
    pub fn create_and_add_node(&mut self, id: &str, label: &str) -> String {
        let node = Node::new(id, label);
        let node_id = node.id.clone();
        self.nodes.insert(node_id.clone(), node);
        node_id
    }
    
    // Commence la construction d'un nouveau nœud
    pub fn create_node_begin(&mut self, id: &str, label: &str) -> String {
        let node_id = id.to_string();
        // On stocke les attributs de base dans un HashMap temporaire
        let mut attributes = HashMap::new();
        attributes.insert("label".to_string(), label.to_string());
        self.nodes_in_construction.insert(node_id.clone(), attributes);
        node_id
    }
    
    // Méthode utilitaire pour manipuler les attributs d'un nœud en construction
    fn with_node_attributes<F, T>(&mut self, id: &str, default: T, f: F) -> T 
    where 
        F: FnOnce(&mut HashMap<String, String>) -> T
    {
        if let Some(attributes) = self.nodes_in_construction.get_mut(id) {
            f(attributes)
        } else {
            default
        }
    }
    
    // Version qui consomme les attributs (pour finish)
    fn with_node_attributes_owned<F, T>(&mut self, id: &str, default: T, f: F) -> T 
    where 
        F: FnOnce(HashMap<String, String>) -> T
    {
        if let Some(attributes) = self.nodes_in_construction.remove(id) {
            f(attributes)
        } else {
            default
        }
    }
    
    // Ajoute un attribut au nœud en construction
    pub fn create_node_add_attribute(&mut self, id: &str, key: &str, value: &str) -> bool {
        self.with_node_attributes(id, false, |attributes| {
            attributes.insert(key.to_string(), value.to_string());
            true
        })
    }
    
    // Finalise la création du nœud
    pub fn create_node_finish(&mut self, id: &str) -> Option<String> {
        self.with_node_attributes_owned(id, None, |attributes| {
            let label = attributes.get("label")
                .cloned()
                .unwrap_or_else(|| id.to_string());
            
            // Créer une copie des attributs sans le label qui est déjà un champ séparé
            let mut node_attributes = attributes.clone();
            node_attributes.remove("label");
            
            // Créer et ajouter le nœud
            let node = Node::with_attributes(id, &label, node_attributes);
            let node_id = node.id.clone();
            self.nodes.insert(node_id.clone(), node);
            
            Some(node_id)
        })
    }

    // Ajoute un tag à un nœud existant
    pub fn add_tag_to_node(&mut self, node_id: &str, tag: &str) -> bool {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.set_attribute("tags", tag);
            true
        } else {
            false
        }
    }

    // Vérifie si un nœud a un tag spécifique
    pub fn node_has_tag(&self, node_id: &str, tag: &str) -> bool {
        self.nodes.get(node_id)
            .map(|node| node.has_tag(tag))
            .unwrap_or(false)
    }

    // Récupère un attribut d'un nœud
    pub fn get_node_attribute(&self, node_id: &str, attribute: &str) -> Option<String> {
        self.nodes.get(node_id)
            .and_then(|node| node.get_attribute(attribute).cloned())
    }
    
    // Récupère le label d'un nœud
    pub fn get_node_label(&self, id: &str) -> Option<String> {
        self.nodes.get(id).map(|node| node.label.clone())
    }

    // Accès aux nœuds
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn node_exists(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn get_all_node_ids(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }
    
    pub fn get_all_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }
}
