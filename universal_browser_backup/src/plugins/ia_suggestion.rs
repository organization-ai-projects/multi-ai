//! # Plugin de suggestions d'IA
//!
//! Ce fichier est responsable de:
//! - Intégrer les suggestions de l'IA dans l'interface
//! - Communiquer avec les services d'IA via le bridge WebSocket
//! - Afficher et permettre l'interaction avec les suggestions
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de rendu de bas niveau
//! - Des définitions de composants génériques
//! - Des logiques non liées aux suggestions d'IA

use crate::layout::{LayoutDirection, LayoutNode, Rect};
use crate::plugins::{Plugin, PluginEvent};
use crate::renderer::Color;
use crate::state::AppState;

pub struct IaSuggestionPlugin {
    suggestions: Vec<String>,
    selected_suggestion: Option<usize>,
    initialized: bool,
}

impl IaSuggestionPlugin {
    pub fn new() -> Self {
        Self {
            suggestions: Vec::new(),
            selected_suggestion: None,
            initialized: false,
        }
    }
}

impl Plugin for IaSuggestionPlugin {
    fn id(&self) -> &str {
        "ia_suggestion"
    }

    fn name(&self) -> &str {
        "IA Suggestions"
    }

    fn initialize(&mut self, _state: &mut AppState) -> Result<(), String> {
        // Initialiser avec quelques suggestions de démonstration
        self.suggestions = vec![
            "Améliorer la structure du projet".to_string(),
            "Optimiser les performances de rendu".to_string(),
            "Ajouter un système de plugins".to_string(),
            "Implémenter la navigation interne".to_string(),
        ];

        self.initialized = true;
        Ok(())
    }

    fn create_layout(&self, width: u32, height: u32) -> LayoutNode {
        let mut panel = LayoutNode::new(
            "ia_suggestion_panel",
            Rect::new(0, 0, width, height),
            LayoutDirection::Vertical,
        )
        .with_background(Color::new(240, 245, 255, 255))
        .with_border(Color::blue());

        // Créer un nœud pour chaque suggestion
        for (i, suggestion) in self.suggestions.iter().enumerate() {
            let mut suggestion_node = LayoutNode::new(
                &format!("suggestion_{}", i),
                Rect::new(10, 10 + i as u32 * 40, width - 20, 30),
                LayoutDirection::Horizontal,
            )
            .with_background(Color::new(220, 230, 250, 255))
            .with_border(Color::new(180, 200, 220, 255));

            suggestion_node.set_focusable(true);

            // Ajouter le nœud à la suggestion principale
            panel.add_child(suggestion_node);
        }

        panel
    }

    fn update(&mut self, _state: &mut AppState) {
        // Mise à jour périodique des suggestions
    }

    fn handle_event(&mut self, event: &PluginEvent, _state: &mut AppState) -> bool {
        match event {
            PluginEvent::WebSocketMessage(msg) => {
                if msg.starts_with("ia_suggestion:") {
                    let suggestion = msg.trim_start_matches("ia_suggestion:").to_string();
                    self.suggestions.push(suggestion);
                    return true;
                }
            }
            _ => {}
        }

        false
    }
}
