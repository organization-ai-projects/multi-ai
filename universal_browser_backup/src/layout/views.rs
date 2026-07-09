//! # Définitions des vues de l'application
//!
//! Ce fichier est responsable de:
//! - Définir les différentes vues de l'application (accueil, IA, graphe, fichiers)
//! - Gérer les transitions entre vues
//! - Construire la structure de chaque vue
//! - Coordonner l'affichage/masquage des panneaux selon la vue active
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Du code de rendu direct
//! - Des définitions détaillées de composants atomiques
//! - Des logiques d'application non liées aux vues
//!
//! Il se concentre sur la définition des vues et leur gestion.

use crate::layout::{components, LayoutDirection, LayoutNode, Rect};
use crate::renderer::Color;
use crate::state::ViewMode;

// Structure pour gérer les vues
pub struct ViewManager {
    // Suppression du champ panels qui n'est jamais lu
}

impl ViewManager {
    pub fn new() -> Self {
        Self {}
    }

    // Obtient l'option de sidebar associée à une vue
    pub fn get_option_id_for_view(&self, view: ViewMode) -> &'static str {
        match view {
            ViewMode::Welcome => "option_welcome",
            ViewMode::IAExplorer => "option_ia",
            ViewMode::GraphMemory => "option_graph",
            ViewMode::FileViewer => "option_files",
        }
    }

    // Ajouter une méthode pour obtenir les IDs de panneaux
    pub fn get_panel_ids() -> [&'static str; 4] {
        ["welcome_panel", "ia_panel", "graph_panel", "files_panel"]
    }
}

/// Crée le panneau de la vue d'accueil
pub fn create_welcome_view(state_width: u32, state_height: u32) -> LayoutNode {
    let mut welcome_panel = components::create_content_panel(
        "welcome_panel",
        Rect::new(220, 60, state_width - 240, state_height - 80),
        LayoutDirection::Vertical,
        true, // Visible par défaut
    );

    // Un élément dans le panneau d'accueil
    let welcome_message = LayoutNode::new(
        "welcome_message",
        Rect::new(250, 100, 400, 100),
        LayoutDirection::Vertical,
    )
    .with_background(Color::new(240, 240, 255, 255))
    .with_border(Color::blue());

    welcome_panel.add_child(welcome_message);
    welcome_panel
}

/// Crée le panneau de la vue IA Explorer
pub fn create_ia_explorer_view(state_width: u32, state_height: u32) -> LayoutNode {
    let mut ia_panel = components::create_content_panel(
        "ia_panel",
        Rect::new(220, 60, state_width - 240, state_height - 80),
        LayoutDirection::Grid(2, 2), // Grille 2x2 pour les IA
        false,                       // Caché initialement
    );

    // Ajouter quelques éléments dans le panneau IA
    for i in 0..4 {
        let ia_card = components::create_card(
            &format!("ia_card_{}", i),
            Rect::new(0, 0, 0, 0), // Dimensions calculées par la grille
            Color::new(230, 250, 230, 255),
            Color::green(),
        );

        ia_panel.add_child(ia_card);
    }

    ia_panel
}

/// Crée le panneau de la vue Graph Memory
pub fn create_graph_memory_view(state_width: u32, state_height: u32) -> LayoutNode {
    let mut graph_panel = components::create_content_panel(
        "graph_panel",
        Rect::new(220, 60, state_width - 240, state_height - 80),
        LayoutDirection::Vertical,
        false, // Caché initialement
    );

    let graph_area = LayoutNode::new(
        "graph_area",
        Rect::new(240, 80, state_width - 280, state_height - 120),
        LayoutDirection::Vertical,
    )
    .with_background(Color::new(250, 240, 230, 255))
    .with_border(Color::new(200, 150, 100, 255));

    graph_panel.add_child(graph_area);
    graph_panel
}

/// Crée le panneau de la vue File Viewer
pub fn create_file_viewer_view(state_width: u32, state_height: u32) -> LayoutNode {
    let mut files_panel = components::create_content_panel(
        "files_panel",
        Rect::new(220, 60, state_width - 240, state_height - 80),
        LayoutDirection::Horizontal,
        false, // Caché initialement
    );

    // Liste de fichiers à gauche
    let file_list = LayoutNode::new(
        "file_list",
        Rect::new(220, 60, 200, state_height - 80),
        LayoutDirection::Vertical,
    )
    .with_background(Color::new(245, 245, 245, 255))
    .with_border(Color::gray(180));

    // Contenu du fichier à droite
    let file_content = LayoutNode::new(
        "file_content",
        Rect::new(420, 60, state_width - 440, state_height - 80),
        LayoutDirection::Vertical,
    )
    .with_background(Color::white())
    .with_border(Color::gray(200));

    files_panel.add_child(file_list);
    files_panel.add_child(file_content);

    files_panel
}
