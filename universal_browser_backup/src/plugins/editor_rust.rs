//! # Plugin d'éditeur Rust
//!
//! Ce fichier est responsable de:
//! - Fournir une interface d'édition pour les fichiers Rust
//! - Gérer la coloration syntaxique
//! - Permettre l'intégration avec les outils Rust (rustfmt, clippy, etc.)
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de rendu de bas niveau
//! - Des définitions de composants génériques
//! - Des logiques non liées à l'édition de code Rust

use crate::layout::{LayoutDirection, LayoutNode, Rect};
use crate::plugins::{Plugin, PluginEvent};
use crate::renderer::Color;
use crate::state::AppState;

pub struct RustEditorPlugin {
    current_file: Option<String>,
    content: String,
    cursor_position: (usize, usize), // ligne, colonne
    initialized: bool,
}

impl RustEditorPlugin {
    pub fn new() -> Self {
        Self {
            current_file: None,
            content: String::new(),
            cursor_position: (0, 0),
            initialized: false,
        }
    }
}

impl Plugin for RustEditorPlugin {
    fn id(&self) -> &str {
        "editor_rust"
    }

    fn name(&self) -> &str {
        "Éditeur Rust"
    }

    fn initialize(&mut self, _state: &mut AppState) -> Result<(), String> {
        // Initialiser avec un exemple de contenu
        self.content = r#"fn main() {
    println!("Hello, Universal Browser!");
}
"#
        .to_string();

        self.initialized = true;
        Ok(())
    }

    fn create_layout(&self, width: u32, height: u32) -> LayoutNode {
        let mut panel = LayoutNode::new(
            "editor_rust_panel",
            Rect::new(0, 0, width, height),
            LayoutDirection::Vertical,
        )
        .with_background(Color::new(250, 250, 250, 255))
        .with_border(Color::new(200, 200, 200, 255));

        // Barre d'onglets
        let tabs = LayoutNode::new(
            "editor_tabs",
            Rect::new(0, 0, width, 30),
            LayoutDirection::Horizontal,
        )
        .with_background(Color::new(230, 230, 230, 255))
        .with_border(Color::new(200, 200, 200, 255));

        panel.add_child(tabs);

        // Zone d'édition
        let editor_area = LayoutNode::new(
            "editor_area",
            Rect::new(0, 30, width, height - 60),
            LayoutDirection::Vertical,
        )
        .with_background(Color::white())
        .with_border(Color::new(200, 200, 200, 255));

        panel.add_child(editor_area);

        // Barre d'état
        let status_bar = LayoutNode::new(
            "editor_status_bar",
            Rect::new(0, height - 30, width, 30),
            LayoutDirection::Horizontal,
        )
        .with_background(Color::new(230, 230, 230, 255));

        panel.add_child(status_bar);

        panel
    }

    fn update(&mut self, _state: &mut AppState) {
        // Mise à jour périodique de l'éditeur
    }

    fn handle_event(&mut self, event: &PluginEvent, _state: &mut AppState) -> bool {
        match event {
            PluginEvent::Navigate(uri) => {
                if uri.starts_with("file://") && uri.ends_with(".rs") {
                    let file_path = uri.trim_start_matches("file://");
                    self.current_file = Some(file_path.to_string());
                    // Dans une implémentation réelle, on chargerait le contenu du fichier ici
                    return true;
                }
            }
            _ => {}
        }

        false
    }
}
