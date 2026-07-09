//liens pour la mémoire graphique de l'ia

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub(crate) struct Link {
    pub(crate) source: String,
    pub(crate) target: String,
    pub(crate) label: Option<String>,
    pub(crate) weight: Option<f32>,
}

impl Link {
    /// Crée un lien simple entre deux nœuds
    pub(crate) fn new(source: &str, target: &str) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            label: None,
            weight: None,
        }
    }

    /// Crée un lien avec tous les détails spécifiés
    pub(crate) fn with_details(source: &str, target: &str, label: Option<String>, weight: Option<f32>) -> Self {
        Self {
            source: source.to_string(),
            target: target.to_string(),
            label,
            weight,
        }
    }
    
    /// Crée un builder pour construire un lien de façon incrémentale
    pub(crate) fn builder(source: &str, target: &str) -> LinkBuilder {
        LinkBuilder::new(source, target)
    }
    
    /// Définit le label du lien (version fluent)
    pub(crate) fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
    
    /// Définit le poids du lien (version fluent)
    pub(crate) fn with_weight(mut self, weight: f32) -> Self {
        self.weight = Some(weight);
        self
    }
}

/// Builder pour construire un lien de façon incrémentale
pub(crate) struct LinkBuilder {
    link: Link,
}

impl LinkBuilder {
    /// Crée un nouveau builder
    pub(crate) fn new(source: &str, target: &str) -> Self {
        Self {
            link: Link::new(source, target)
        }
    }
    
    /// Ajoute un label au lien en construction
    pub(crate) fn with_label(mut self, label: &str) -> Self {
        self.link.label = Some(label.to_string());
        self
    }
    
    /// Ajoute un poids au lien en construction
    pub(crate) fn with_weight(mut self, weight: f32) -> Self {
        self.link.weight = Some(weight);
        self
    }
    
    /// Construit le lien final
    pub(crate) fn build(self) -> Link {
        self.link
    }
}