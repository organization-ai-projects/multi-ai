//! # Module de gestion du layout de l'interface utilisateur
//!
//! Ce module est responsable de:
//! - Définir la structure hiérarchique de l'interface
//! - Gérer le calcul des positions et tailles des éléments
//! - Fournir des composants réutilisables
//! - Organiser les différentes vues de l'application
//!
//! Ce module est divisé en sous-modules spécialisés:
//! - rect.rs: Définition des rectangles de base
//! - node.rs: Nœuds de l'arbre de layout
//! - manager.rs: Gestion globale du layout
//! - components.rs: Composants d'UI réutilisables
//! - views.rs: Définition des différentes vues
//! - presets.rs: Configurations prédéfinies de layout

// Réexporte les structures publiques
mod components;
mod manager;
mod node;
mod presets;
mod rect;
pub mod views;

pub use manager::LayoutManager;
pub use node::{LayoutDirection, LayoutNode};
pub use presets::{create_default_layout, update_panel_visibility};
pub use rect::Rect;
