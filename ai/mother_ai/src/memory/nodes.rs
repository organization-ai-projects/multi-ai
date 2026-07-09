//noeuds de la mémoire graphique de l'ia
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Node {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) attributes: Option<HashMap<String, String>>,
}

impl Node {
    /// Crée un nouveau nœud avec l'ID et le label spécifiés
    pub(crate) fn new(id: &str, label: &str) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            attributes: None,
        }
    }

    /// Crée un nouveau nœud avec des attributs
    pub(crate) fn with_attributes(id: &str, label: &str, attributes: HashMap<String, String>) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            attributes: Some(attributes),
        }
    }
    
    /// Crée un builder pour construire un nœud de façon incrémentale
    pub(crate) fn builder(id: &str, label: &str) -> NodeBuilder {
        NodeBuilder::new(id, label)
    }
    
    /// Helper pour récupérer un attribut spécifique
    pub(crate) fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes.as_ref().and_then(|attrs| attrs.get(key))
    }
    
    /// Helper pour définir un attribut
    pub(crate) fn set_attribute(&mut self, key: &str, value: &str) -> &mut Self {
        let attrs = self.attributes.get_or_insert_with(HashMap::new);
        attrs.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Ajoute des tags au nœud (version fluent)
    pub(crate) fn with_tags(mut self, tags: Vec<String>) -> Self {
        if self.attributes.is_none() {
            self.attributes = Some(HashMap::new());
        }
        
        if let Some(attrs) = &mut self.attributes {
            attrs.insert("tags".to_string(), tags.join(","));
        }
        
        self
    }
    
    /// Vérifie si le nœud possède un tag spécifique
    pub(crate) fn has_tag(&self, tag: &str) -> bool {
        self.get_attribute("tags")
            .map(|tags| tags.split(',').any(|t| t == tag))
            .unwrap_or(false)
    }
}

/// Builder pour construire un nœud de façon incrémentale
pub(crate) struct NodeBuilder {
    node: Node,
}

impl NodeBuilder {
    /// Crée un nouveau builder
    pub(crate) fn new(id: &str, label: &str) -> Self {
        Self {
            node: Node::new(id, label)
        }
    }
    
    /// Ajoute un attribut au nœud en construction
    pub(crate) fn with_attribute(mut self, key: &str, value: &str) -> Self {
        let attrs = self.node.attributes.get_or_insert_with(HashMap::new);
        attrs.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Ajoute des tags au nœud en construction
    pub(crate) fn with_tags(mut self, tags: Vec<String>) -> Self {
        let attrs = self.node.attributes.get_or_insert_with(HashMap::new);
        attrs.insert("tags".to_string(), tags.join(","));
        self
    }
    
    /// Construit le nœud final
    pub(crate) fn build(self) -> Node {
        self.node
    }
}