//! # Système de navigation interne
//!
//! Ce fichier est responsable de:
//! - Gérer les URL/URI internes de l'application
//! - Permettre la navigation entre différentes ressources
//! - Parser et valider les adresses
//! - Déclencher les changements de vue appropriés
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de rendu
//! - Des définitions de layout
//! - Des opérations sur l'état global (seulement des requêtes)
//!
//! Il implémente un système d'adressage uniforme pour toutes les ressources.

use crate::state::{AppState, ViewMode};

/// Structure représentant une URI interne
#[derive(Debug, Clone, PartialEq)]
pub struct InternalUri {
    /// Le protocole (module) : "graph", "ia", "file", etc.
    pub protocol: String,

    /// Le chemin de la ressource
    pub path: String,

    /// Les paramètres optionnels
    pub params: Vec<(String, String)>,
}

impl InternalUri {
    /// Parse une chaîne en URI interne
    pub fn parse(uri: &str) -> Result<Self, String> {
        // Format attendu: "protocol://path?param1=value1&param2=value2"
        let parts: Vec<&str> = uri.split("://").collect();
        if parts.len() != 2 {
            return Err(format!("URI invalide: {}", uri));
        }

        let protocol = parts[0].to_string();

        // Séparer le chemin des paramètres
        let path_parts: Vec<&str> = parts[1].split('?').collect();
        let path = path_parts[0].to_string();

        // Parser les paramètres s'ils existent
        let mut params = Vec::new();
        if path_parts.len() > 1 {
            for param_str in path_parts[1].split('&') {
                let param_parts: Vec<&str> = param_str.split('=').collect();
                if param_parts.len() == 2 {
                    params.push((param_parts[0].to_string(), param_parts[1].to_string()));
                }
            }
        }

        Ok(InternalUri {
            protocol,
            path,
            params,
        })
    }

    /// Convertit l'URI interne en chaîne
    pub fn to_string(&self) -> String {
        let mut result = format!("{}://{}", self.protocol, self.path);

        if !self.params.is_empty() {
            result.push('?');
            for (i, (key, value)) in self.params.iter().enumerate() {
                if i > 0 {
                    result.push('&');
                }
                result.push_str(&format!("{}={}", key, value));
            }
        }

        result
    }
}

/// Gestionnaire de navigation
pub struct Navigator {
    /// Historique de navigation
    pub history: Vec<InternalUri>, // Rendre l'historique public

    /// Position actuelle dans l'historique
    pub current_index: usize, // Rendre l'index courant public

    /// Adresse actuelle
    pub current_uri: Option<InternalUri>, // Rendre l'URI courante publique
}

impl Navigator {
    /// Crée un nouveau navigateur
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            current_index: 0,
            current_uri: None,
        }
    }

    /// Navigue vers une nouvelle adresse
    pub fn goto(&mut self, uri_str: &str, state: &mut AppState) -> Result<(), String> {
        let uri = InternalUri::parse(uri_str)?;

        // Si l'URI est différente de l'actuelle, l'ajouter à l'historique
        if Some(&uri) != self.current_uri.as_ref() {
            // Tronquer l'historique si on navigue depuis une position intermédiaire
            if self.current_index < self.history.len() {
                self.history.truncate(self.current_index + 1);
            }

            self.history.push(uri.clone());
            self.current_index = self.history.len() - 1;
            self.current_uri = Some(uri.clone());
        }

        // Changer la vue en fonction du protocole
        match uri.protocol.as_str() {
            "welcome" => state.switch_view(ViewMode::Welcome),
            "ia" => state.switch_view(ViewMode::IAExplorer),
            "graph" => state.switch_view(ViewMode::GraphMemory),
            "file" => state.switch_view(ViewMode::FileViewer),
            _ => return Err(format!("Protocole inconnu: {}", uri.protocol)),
        }

        Ok(())
    }

    /// Revient en arrière dans l'historique
    pub fn back(&mut self, state: &mut AppState) -> Result<(), String> {
        if self.current_index > 0 {
            self.current_index -= 1;
            let uri = self.history[self.current_index].clone();
            self.current_uri = Some(uri.clone());

            // Changer la vue en fonction du protocole
            match uri.protocol.as_str() {
                "welcome" => state.switch_view(ViewMode::Welcome),
                "ia" => state.switch_view(ViewMode::IAExplorer),
                "graph" => state.switch_view(ViewMode::GraphMemory),
                "file" => state.switch_view(ViewMode::FileViewer),
                _ => return Err(format!("Protocole inconnu: {}", uri.protocol)),
            }

            Ok(())
        } else {
            Err("Impossible de revenir en arrière".to_string())
        }
    }

    /// Avance dans l'historique
    pub fn forward(&mut self, state: &mut AppState) -> Result<(), String> {
        if self.current_index < self.history.len() - 1 {
            self.current_index += 1;
            let uri = self.history[self.current_index].clone();
            self.current_uri = Some(uri.clone());

            // Changer la vue en fonction du protocole
            match uri.protocol.as_str() {
                "welcome" => state.switch_view(ViewMode::Welcome),
                "ia" => state.switch_view(ViewMode::IAExplorer),
                "graph" => state.switch_view(ViewMode::GraphMemory),
                "file" => state.switch_view(ViewMode::FileViewer),
                _ => return Err(format!("Protocole inconnu: {}", uri.protocol)),
            }

            Ok(())
        } else {
            Err("Impossible d'avancer".to_string())
        }
    }

    /// Retourne l'URI actuelle
    pub fn current_uri(&self) -> Option<&InternalUri> {
        self.current_uri.as_ref()
    }

    /// Retourne l'index courant dans l'historique
    pub fn get_current_index(&self) -> usize {
        self.current_index
    }

    /// Retourne une référence à l'historique
    pub fn get_history(&self) -> &[InternalUri] {
        &self.history
    }

    /// Modifie l'index courant et l'URI courante
    pub fn set_current_index(&mut self, index: usize) -> Result<(), String> {
        if index >= self.history.len() {
            return Err("Index invalide".to_string());
        }

        self.current_index = index;
        self.current_uri = Some(self.history[index].clone());
        Ok(())
    }
}

/// Fonctions utilitaires pour la navigation
pub mod presets {
    use super::*;

    /// Navigue vers la page d'accueil
    pub fn goto_welcome(navigator: &mut Navigator, state: &mut AppState) -> Result<(), String> {
        navigator.goto("welcome://home", state)
    }

    /// Navigue vers l'explorateur d'IA
    pub fn goto_ia_explorer(navigator: &mut Navigator, state: &mut AppState) -> Result<(), String> {
        navigator.goto("ia://explorer", state)
    }

    /// Navigue vers les suggestions d'IA
    pub fn goto_ia_suggestions(
        navigator: &mut Navigator,
        state: &mut AppState,
    ) -> Result<(), String> {
        navigator.goto("ia://suggestions", state)
    }

    /// Navigue vers la mémoire graphique
    pub fn goto_graph_memory(
        navigator: &mut Navigator,
        state: &mut AppState,
    ) -> Result<(), String> {
        navigator.goto("graph://memory", state)
    }

    /// Navigue vers la dernière version du graphe
    pub fn goto_graph_latest(
        navigator: &mut Navigator,
        state: &mut AppState,
    ) -> Result<(), String> {
        navigator.goto("graph://latest", state)
    }

    /// Navigue vers l'explorateur de fichiers
    pub fn goto_file_explorer(
        navigator: &mut Navigator,
        state: &mut AppState,
    ) -> Result<(), String> {
        navigator.goto("file://explorer", state)
    }

    /// Navigue vers un fichier spécifique
    pub fn goto_file(
        navigator: &mut Navigator,
        path: &str,
        state: &mut AppState,
    ) -> Result<(), String> {
        navigator.goto(&format!("file://{}", path), state)
    }
}
