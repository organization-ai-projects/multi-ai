//! # Plugin de graphe de versionnement
//!
//! Ce fichier est responsable de:
//! - Visualiser les graphes de dépendances et d'historique
//! - Gérer les interactions avec le graphe
//! - Communiquer avec le système de versionnement
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de rendu de bas niveau
//! - Des définitions de composants génériques
//! - Des logiques non liées au graphe de versionnement

use crate::layout::{LayoutDirection, LayoutNode, Rect};
use crate::plugins::{Plugin, PluginEvent};
use crate::renderer::Color;
use crate::state::AppState;

pub struct VersioningGraphPlugin {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    selected_node: Option<usize>,
    initialized: bool,
}

struct GraphNode {
    id: String,
    label: String,
    x: f32,
    y: f32,
}

struct GraphEdge {
    from_id: String,
    to_id: String,
}

impl VersioningGraphPlugin {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            selected_node: None,
            initialized: false,
        }
    }
}

impl Plugin for VersioningGraphPlugin {
    fn id(&self) -> &str {
        "versioning_graph"
    }

    fn name(&self) -> &str {
        "Graphe de Versionnement"
    }

    fn initialize(&mut self, _state: &mut AppState) -> Result<(), String> {
        // Initialiser avec quelques nœuds et arêtes de démonstration
        self.nodes = vec![
            GraphNode {
                id: "v1".to_string(),
                label: "Version 1".to_string(),
                x: 100.0,
                y: 100.0,
            },
            GraphNode {
                id: "v2".to_string(),
                label: "Version 2".to_string(),
                x: 200.0,
                y: 100.0,
            },
            GraphNode {
                id: "v3".to_string(),
                label: "Version 3".to_string(),
                x: 300.0,
                y: 100.0,
            },
            GraphNode {
                id: "branch1".to_string(),
                label: "Branch 1".to_string(),
                x: 200.0,
                y: 200.0,
            },
        ];

        self.edges = vec![
            GraphEdge {
                from_id: "v1".to_string(),
                to_id: "v2".to_string(),
            },
            GraphEdge {
                from_id: "v2".to_string(),
                to_id: "v3".to_string(),
            },
            GraphEdge {
                from_id: "v2".to_string(),
                to_id: "branch1".to_string(),
            },
        ];

        self.initialized = true;
        Ok(())
    }

    fn create_layout(&self, width: u32, height: u32) -> LayoutNode {
        let mut panel = LayoutNode::new(
            "versioning_graph_panel",
            Rect::new(0, 0, width, height),
            LayoutDirection::Vertical,
        )
        .with_background(Color::new(245, 245, 250, 255))
        .with_border(Color::new(200, 200, 220, 255));

        // Créer un nœud pour la zone de graphe
        let graph_area = LayoutNode::new(
            "graph_area",
            Rect::new(10, 10, width - 20, height - 60),
            LayoutDirection::Vertical,
        )
        .with_background(Color::new(255, 255, 255, 255))
        .with_border(Color::new(180, 180, 200, 255));

        panel.add_child(graph_area);

        // Créer une barre d'outils
        let toolbar = LayoutNode::new(
            "graph_toolbar",
            Rect::new(10, height - 40, width - 20, 30),
            LayoutDirection::Horizontal,
        )
        .with_background(Color::new(220, 220, 235, 255));

        panel.add_child(toolbar);

        panel
    }

    fn update(&mut self, _state: &mut AppState) {
        // Mise à jour périodique du graphe
    }

    fn handle_event(&mut self, event: &PluginEvent, _state: &mut AppState) -> bool {
        match event {
            PluginEvent::WebSocketMessage(msg) => {
                if msg.starts_with("graph:") {
                    // Traiter les messages spécifiques au graphe
                    return true;
                }
            }
            _ => {}
        }

        false
    }
}
