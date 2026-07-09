//! # Interface de plugin
//!
//! Ce fichier est responsable de:
//! - Définir l'interface commune à tous les plugins
//! - Spécifier les méthodes que les plugins doivent implémenter
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des implémentations concrètes de plugins
//! - De la logique de gestion des plugins
//!
//! Il définit uniquement le contrat que les plugins doivent respecter.

use super::event::PluginEvent;
use crate::layout::{LayoutNode, Rect};
use crate::state::AppState;

/// Trait définissant l'interface d'un plugin
pub trait Plugin {
    /// Retourne l'identifiant unique du plugin
    fn id(&self) -> &str;

    /// Retourne le nom d'affichage du plugin
    fn name(&self) -> &str;

    /// Initialise le plugin
    fn initialize(&mut self, state: &mut AppState) -> Result<(), String>;

    /// Crée le nœud de layout pour le plugin
    fn create_layout(&self, width: u32, height: u32) -> LayoutNode;

    /// Met à jour l'état du plugin
    fn update(&mut self, state: &mut AppState);

    /// Traite un événement pour le plugin
    fn handle_event(&mut self, event: &PluginEvent, state: &mut AppState) -> bool;
}
