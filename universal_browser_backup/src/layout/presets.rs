//! # Configurations prédéfinies de layout
//!
//! Ce fichier est responsable de:
//! - Définir des configurations complètes de layout
//! - Assembler les composants pour former l'interface complète
//! - Initialiser le layout au démarrage de l'application
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des définitions de composants atomiques
//! - Du code de rendu direct
//! - Des logiques de gestion d'événements
//!
//! Il se concentre sur l'assemblage des layouts complexes à partir des composants.

use crate::layout::{components, views, LayoutDirection, LayoutNode, Rect};
use crate::renderer::Color;
use crate::state::{AppState, ViewMode};

/// Crée la structure de layout initiale pour l'application
pub fn create_default_layout(state: &mut AppState) {
    // Créer le layout de base
    let mut root = LayoutNode::new(
        "root",
        Rect::new(0, 0, state.size.width, state.size.height),
        LayoutDirection::Vertical,
    );

    // Créer l'en-tête
    let header = create_header_section(state.size.width);
    root.add_child(header);

    // Créer la zone principale (sidebar + contenu)
    let main_area = create_main_area(state.size.width, state.size.height);
    root.add_child(main_area);

    // Calculer le layout final
    state.layout_manager.root = root;
    state.layout_manager.calculate_layout();

    // Mettre à jour la visibilité des panneaux en fonction de la vue actuelle
    // On ne peut pas utiliser state.view_manager car cela crée un double emprunt
    update_panel_visibility(state);
}

/// Crée la section d'en-tête avec ses boutons
fn create_header_section(width: u32) -> LayoutNode {
    // En-tête
    let mut header = LayoutNode::new(
        "header",
        Rect::new(0, 0, width, 40),
        LayoutDirection::Horizontal,
    )
    .with_background(Color::new(100, 149, 237, 255));

    // Ajouter des boutons dans l'en-tête avec une grille
    let mut header_buttons = LayoutNode::new(
        "header_buttons",
        Rect::new(10, 5, 400, 30),
        LayoutDirection::Grid(4, 1), // 4 boutons sur 1 ligne
    );

    // Créer les boutons et les ajouter à la grille
    header_buttons.add_child(components::create_button("btn_home", Rect::new(0, 0, 0, 0)));
    header_buttons.add_child(components::create_button(
        "btn_explorer",
        Rect::new(0, 0, 0, 0),
    ));
    header_buttons.add_child(components::create_button(
        "btn_graph",
        Rect::new(0, 0, 0, 0),
    ));
    header_buttons.add_child(components::create_button(
        "btn_files",
        Rect::new(0, 0, 0, 0),
    ));

    // Ajouter la grille à l'en-tête
    header.add_child(header_buttons);

    header
}

/// Crée la zone principale avec sidebar et contenu
fn create_main_area(width: u32, height: u32) -> LayoutNode {
    // Zone principale (contient sidebar + content)
    let mut main_area = LayoutNode::new(
        "main_area",
        Rect::new(0, 40, width, height - 40),
        LayoutDirection::Horizontal,
    );

    // Sidebar avec ses options
    let sidebar = create_sidebar(width, height);

    // Zone de contenu principale
    let mut content = LayoutNode::new(
        "content",
        Rect::new(200, 40, width - 200, height - 40),
        LayoutDirection::Vertical,
    )
    .with_background(Color::white())
    .with_border(Color::new(200, 200, 200, 255));

    // Ajouter les différentes vues
    content.add_child(views::create_welcome_view(width, height));
    content.add_child(views::create_ia_explorer_view(width, height));
    content.add_child(views::create_graph_memory_view(width, height));
    content.add_child(views::create_file_viewer_view(width, height));

    main_area.add_child(sidebar);
    main_area.add_child(content);

    main_area
}

/// Crée la sidebar avec ses options
fn create_sidebar(width: u32, height: u32) -> LayoutNode {
    // Utiliser la variable width pour calculer la largeur de la sidebar
    // Par exemple, 15% de la largeur totale, avec un minimum de 200
    let sidebar_width = (width * 15 / 100).max(200);

    // Sidebar avec une grille pour les options
    let mut sidebar = LayoutNode::new(
        "sidebar",
        Rect::new(0, 40, sidebar_width, height - 40),
        LayoutDirection::Vertical,
    )
    .with_background(Color::new(220, 220, 220, 255));

    // Ajouter une grille pour les boutons de la sidebar
    let mut sidebar_buttons = LayoutNode::new(
        "sidebar_buttons",
        Rect::new(10, 50, sidebar_width - 20, 200),
        LayoutDirection::Grid(1, 4), // 1 colonne, 4 lignes
    );

    // Ajouter les options à la grille
    sidebar_buttons.add_child(components::create_sidebar_option(
        "option_welcome",
        Rect::new(0, 0, 0, 0),
    ));
    sidebar_buttons.add_child(components::create_sidebar_option(
        "option_ia",
        Rect::new(0, 0, 0, 0),
    ));
    sidebar_buttons.add_child(components::create_sidebar_option(
        "option_graph",
        Rect::new(0, 0, 0, 0),
    ));
    sidebar_buttons.add_child(components::create_sidebar_option(
        "option_files",
        Rect::new(0, 0, 0, 0),
    ));

    // Ajouter la grille à la sidebar
    sidebar.add_child(sidebar_buttons);

    sidebar
}

/// Met à jour la visibilité des panneaux en fonction de la vue active
pub fn update_panel_visibility(state: &mut AppState) {
    // Copier la vue actuelle pour éviter d'emprunter state plus tard
    let current_view = state.current_view;

    // Utiliser la méthode statique pour obtenir les IDs de panneaux
    let panel_ids = views::ViewManager::get_panel_ids();

    let should_be_visible = [
        current_view == ViewMode::Welcome,
        current_view == ViewMode::IAExplorer,
        current_view == ViewMode::GraphMemory,
        current_view == ViewMode::FileViewer,
    ];

    // Appliquer les changements de visibilité
    for (id, &visible) in panel_ids.iter().zip(should_be_visible.iter()) {
        if let Some(panel) = state.layout_manager.root.find_node_by_id_mut(id) {
            panel.set_visible(visible);
        }
    }
}
