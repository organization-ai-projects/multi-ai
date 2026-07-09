// memory/mod.rs
//! Gestion de la mémoire graphique de l'IA
//!
//! - `node.rs` : définition interne des nœuds
//! - `links.rs` : liens entre nœuds
//! - `nodes_manager.rs` : gestionnaire des opérations sur les nœuds
//! - `links_manager.rs` : gestionnaire des opérations sur les liens 
//! - `graph_memory_manager.rs` : coordination des opérations sur le graphe
//! - `memory_storage.rs` : interface I/O publique du domaine
//! - `path_manager.rs` : gestion des chemins de mémoire

// 📦 Modules internes avec visibilité limitée au crate
pub(crate) mod node;
pub(crate) mod links;
pub(crate) mod nodes_manager;
pub(crate) mod links_manager;
pub(crate) mod graph_memory_manager;
pub(crate) mod path_manager;

// 🚪 Seule porte d'entrée publique du domaine mémoire
pub mod memory_storage;