//! Module de gestion du graphe mémoire pour l'IA : permet de représenter, organiser et valider des souvenirs sous forme de nœuds et de liens.

pub(crate) mod link;
pub(crate) mod links;
pub(crate) mod links_manager;
pub(crate) mod node;
pub(crate) mod nodes;
pub(crate) mod nodes_manager;
pub(crate) mod memory_graph;
pub(crate) mod persist; // Restriction de visibilité
pub mod persist_memory_graph; // Seul ce module est exposé à l'extérieur
