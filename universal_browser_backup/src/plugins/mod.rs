//! # Système de plugins
//!
//! Ce module est responsable de:
//! - Organiser l'architecture des plugins
//! - Exporter les interfaces communes aux plugins
//!
//! Ce module NE DOIT PAS contenir:
//! - Des implémentations concrètes de plugins
//! - Des définitions détaillées d'interfaces
//! - De la logique métier
//!
//! Il sert uniquement de point d'entrée pour le système de plugins.

mod editor_rust;
mod event;
mod ia_suggestion;
mod interface;
mod manager;
mod memory_explorer;
mod versioning_graph;

// Réexportation des structures publiques
pub use event::PluginEvent;
pub use interface::Plugin;
pub use manager::PluginManager;
