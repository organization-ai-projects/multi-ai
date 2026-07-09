//! # Gestion des événements de plugins
//!
//! Ce fichier est responsable de:
//! - Activer ou désactiver les plugins
//! - Gérer les événements spécifiques aux plugins
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de navigation
//! - Des gestionnaires d'entrées utilisateur
//!
//! Il se concentre uniquement sur les événements liés aux plugins.

use crate::plugins::PluginEvent;
use crate::state::AppState;

pub fn activate_plugin(id: &str, state: &mut AppState) -> Result<(), String> {
    let result = state.plugin_manager.activate_plugin(id, state);
    if result.is_ok() {
        state.plugin_manager.queue_event(PluginEvent::Activated);
    }
    result
}
