//! # Module de rendu graphique
//!
//! Ce module est responsable de:
//! - Coordonner le rendu de l'interface utilisateur
//! - Fournir des primitives de dessin
//! - Gérer les couleurs et les styles visuels
//!
//! Ce module est divisé en sous-modules spécialisés:
//! - color.rs: Définition et gestion des couleurs
//! - primitives.rs: Fonctions de dessin de base
//! - core.rs: Implémentation principale du renderer
//! - view_renderers.rs: Rendu des différentes vues
//! - layout_renderer.rs: Rendu du système de layout

// Module de rendu graphique - Point d'entrée
mod color;
mod core;
mod layout_renderer;
mod primitives;
mod view_renderers;

// Réexportation des structures publiques
pub use color::Color;
pub use core::Renderer;
