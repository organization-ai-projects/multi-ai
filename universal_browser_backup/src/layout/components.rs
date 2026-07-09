//! # Composants d'interface utilisateur réutilisables
//! 
//! Ce fichier est responsable de:
//! - Définir des fonctions de création de composants standards
//! - Assurer la cohérence visuelle des éléments similaires
//! - Faciliter la création d'éléments d'interface courants
//! 
//! Ce fichier NE DOIT PAS contenir:
//! - Des définitions de vues complètes
//! - Des logiques de gestion d'événements
//! - Du code de calcul de layout
//! 
//! Il fournit une bibliothèque de composants réutilisables pour construire l'interface.

use crate::layout::{LayoutDirection, LayoutNode, Rect};
use crate::renderer::Color;

/// Crée un bouton standard
pub fn create_button(id: &str, rect: Rect) -> LayoutNode {
    let mut button = LayoutNode::new(id, rect, LayoutDirection::Horizontal)
        .with_background(Color::new(80, 120, 200, 255))
        .with_border(Color::white());

    button.set_focusable(true);
    button
}

/// Crée une option de sidebar
pub fn create_sidebar_option(id: &str, rect: Rect) -> LayoutNode {
    let mut option = LayoutNode::new(id, rect, LayoutDirection::Horizontal)
        .with_background(Color::new(180, 180, 180, 255))
        .with_border(Color::new(150, 150, 150, 255));

    option.set_focusable(true);
    option
}

/// Crée un panneau de contenu
pub fn create_content_panel(
    id: &str,
    rect: Rect,
    layout_dir: LayoutDirection,
    visible: bool,
) -> LayoutNode {
    let mut panel = LayoutNode::new(id, rect, layout_dir);
    panel.set_visible(visible);
    panel
}

/// Crée une carte pour affichage dans une grille
pub fn create_card(id: &str, rect: Rect, color: Color, border_color: Color) -> LayoutNode {
    LayoutNode::new(id, rect, LayoutDirection::Vertical)
        .with_background(color)
        .with_border(border_color)
}
