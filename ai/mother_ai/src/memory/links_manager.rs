//! Gestionnaire des liens du graphe mémoire
//!
//! Ce module est responsable de toutes les opérations sur les liens,
//! telles que création, mise à jour, filtrage et désactivation.
//! il utilise uniquement links.rs

use std::collections::HashMap;
use super::links::link::Link;

/// Structure gérant les opérations sur les liens
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct LinksManager {
    links: Vec<Link>,
}

impl LinksManager {
    //
    // INITIALISATION
    //
    
    /// Crée un nouveau gestionnaire de liens vide
    pub(crate) fn new() -> Self {
        Self {
            links: Vec::new(),
        }
    }
    
    //
    // OPÉRATIONS DE BASE (CRUD)
    //
    
    /// Crée un nouveau lien avec les détails spécifiés
    pub(crate) fn create_link(&self, source: &str, target: &str, label: Option<String>, weight: Option<f32>) -> Link {
        Link::with_details(source, target, label, weight)
    }
    
    /// Ajoute un lien à la collection
    pub(crate) fn add_link(&mut self, link: Link) {
        self.links.push(link);
    }

    /// Récupère un lien spécifique entre deux nœuds
    pub(crate) fn get_link(&self, source: &str, target: &str) -> Option<&Link> {
        self.links
            .iter()
            .find(|link| link.source == source && link.target == target)
    }
    
    /// Désactive un lien au lieu de le supprimer
    pub(crate) fn disable_link(&mut self, source: &str, target: &str) -> bool {
        if let Some(link) = self.links
            .iter_mut()
            .find(|link| link.source == source && link.target == target)
        {
            link.weight = Some(0.0);  // Poids zéro pour indiquer un lien inactif
            true
        } else {
            false
        }
    }

    //
    // OPÉRATIONS DE MISE À JOUR
    //
    
    /// Met à jour les propriétés d'un lien
    pub(crate) fn update_link(&mut self, source: &str, target: &str, label: Option<String>, weight: Option<f32>) -> bool {
        if let Some(link) = self.links
            .iter_mut()
            .find(|link| link.source == source && link.target == target) 
        {
            link.label = label;
            link.weight = weight;
            true
        } else {
            false
        }
    }
    
    //
    // OPÉRATIONS DE RECHERCHE ET FILTRAGE
    //
    
    /// Récupère tous les liens liés à un nœud spécifique
    pub(crate) fn get_links_for_node(&self, node_id: &str) -> Vec<&Link> {
        self.links.iter()
            .filter(|link| link.source == node_id || link.target == node_id)
            .collect()
    }

    /// Filtre les liens actifs (poids > 0)
    pub(crate) fn get_active_links(&self) -> Vec<&Link> {
        self.links
            .iter()
            .filter(|link| link.weight.map_or(true, |w| w > 0.0))
            .collect()
    }

    /// Retourne tous les liens du gestionnaire
    pub(crate) fn get_all_links(&self) -> &[Link] {
        &self.links
    }
    
    //
    // ACCESSEURS POUR GRAPH_MEMORY_MANAGER
    //
    
    /// Exposer le Vec interne pour GraphMemoryManager
    pub(crate) fn get_links_vec(&self) -> &Vec<Link> {
        &self.links
    }
    
    /// Exposer le Vec interne mutable pour GraphMemoryManager
    pub(crate) fn get_links_vec_mut(&mut self) -> &mut Vec<Link> {
        &mut self.links
    }
}
