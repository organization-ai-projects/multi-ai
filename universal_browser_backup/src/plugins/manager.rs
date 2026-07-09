//! # Gestionnaire de plugins
//!
//! Ce fichier est responsable de:
//! - Charger et gérer les plugins
//! - Distribuer les événements aux plugins appropriés
//! - Coordonner les interactions entre plugins
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des définitions d'interfaces de plugins
//! - Des implémentations spécifiques de plugins
//! - Du code de rendu direct
//!
//! Il se concentre sur la gestion du cycle de vie des plugins.

use crate::plugins::{
    editor_rust::RustEditorPlugin, ia_suggestion::IaSuggestionPlugin,
    memory_explorer::MemoryExplorerPlugin, versioning_graph::VersioningGraphPlugin, Plugin,
    PluginEvent,
};
use crate::state::AppState;
use std::collections::VecDeque;

/// Gestionnaire de plugins
pub struct PluginManager {
    /// Les plugins chargés
    plugins: Vec<Box<dyn Plugin>>,

    /// L'ID du plugin actif
    active_plugin_id: Option<String>,

    /// File d'attente d'événements à traiter
    event_queue: VecDeque<PluginEvent>,
}

impl PluginManager {
    /// Crée un nouveau gestionnaire de plugins
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            active_plugin_id: None,
            event_queue: VecDeque::new(),
        }
    }

    /// Charge tous les plugins intégrés
    pub fn load_builtin_plugins(&mut self, state: &mut AppState) -> Result<(), String> {
        // Charger le plugin de suggestions d'IA
        let mut plugin = IaSuggestionPlugin::new();
        plugin.initialize(state)?;
        self.plugins.push(Box::new(plugin));

        // Charger le plugin de versioning de graphe
        let mut plugin = VersioningGraphPlugin::new();
        plugin.initialize(state)?;
        self.plugins.push(Box::new(plugin));

        // Charger le plugin d'éditeur Rust
        let mut plugin = RustEditorPlugin::new();
        plugin.initialize(state)?;
        self.plugins.push(Box::new(plugin));

        // Charger le plugin d'explorateur de mémoire
        let mut plugin = MemoryExplorerPlugin::new();
        plugin.initialize(state)?;
        self.plugins.push(Box::new(plugin));

        Ok(())
    }

    /// Active un plugin par son ID
    pub fn activate_plugin(&mut self, id: &str, state: &mut AppState) -> Result<(), String> {
        // Désactiver le plugin actif
        if let Some(active_id) = &self.active_plugin_id {
            if let Some(plugin) = self.get_plugin_mut(active_id) {
                plugin.handle_event(&PluginEvent::Deactivated, state);
            }
        }

        // Activer le nouveau plugin
        if let Some(plugin) = self.get_plugin_mut(id) {
            plugin.handle_event(&PluginEvent::Activated, state);
            self.active_plugin_id = Some(id.to_string());
            Ok(())
        } else {
            Err(format!("Plugin non trouvé: {}", id))
        }
    }

    /// Obtient un plugin par son ID
    pub fn get_plugin(&self, id: &str) -> Option<&dyn Plugin> {
        self.plugins
            .iter()
            .find(|p| p.id() == id)
            .map(|p| p.as_ref())
    }

    /// Obtient un plugin mutable par son ID
    pub fn get_plugin_mut(&mut self, id: &str) -> Option<&mut dyn Plugin> {
        self.plugins
            .iter_mut()
            .find(|p| p.id() == id)
            .map(|p| p.as_mut())
    }

    /// Traite un événement pour tous les plugins
    pub fn handle_event(&mut self, event: PluginEvent, state: &mut AppState) {
        // Si un plugin actif est défini, lui envoyer l'événement en priorité
        if let Some(active_id) = &self.active_plugin_id {
            if let Some(plugin) = self.get_plugin_mut(active_id) {
                if plugin.handle_event(&event, state) {
                    return; // L'événement a été traité
                }
            }
        }

        // Sinon, envoyer l'événement à tous les plugins
        for plugin in &mut self.plugins {
            if plugin.handle_event(&event, state) {
                break; // L'événement a été traité
            }
        }
    }

    /// Ajoute un événement à la file d'attente pour traitement ultérieur
    pub fn queue_event(&mut self, event: PluginEvent) {
        self.event_queue.push_back(event);
    }

    /// Met à jour tous les plugins et traite les événements en attente
    pub fn update_all(&mut self, state: &mut AppState) {
        // Traiter les événements en attente
        while let Some(event) = self.event_queue.pop_front() {
            self.handle_event(event, state);
        }

        // Mettre à jour les plugins
        for plugin in &mut self.plugins {
            plugin.update(state);
        }
    }

    /// Retourne tous les plugins
    pub fn get_all_plugins(&self) -> &[Box<dyn Plugin>] {
        &self.plugins
    }
}
