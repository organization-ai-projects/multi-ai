//! # Rendu du système de layout
//!
//! Ce fichier est responsable de:
//! - Dessiner les nœuds du layout et leur hiérarchie
//! - Gérer l'affichage des éléments focusés
//! - Appliquer les styles visuels aux composants UI
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de calcul de layout
//! - Des définitions spécifiques aux vues
//! - Des primitives de dessin génériques
//!
//! Il se concentre sur la représentation visuelle du layout.

use super::{color::Color, primitives::draw_rect};
use crate::layout::{LayoutManager, LayoutNode};

/// Fonction principale pour le rendu du layout
pub fn render_layout(layout_manager: &LayoutManager, frame: &mut [u8], width: u32, height: u32) {
    // Parcourir tous les nœuds et les dessiner manuellement
    render_node(
        &layout_manager.root,
        frame,
        width,
        height,
        &layout_manager.focused_id,
    );
}

/// Fonction récursive pour dessiner un nœud et ses enfants
pub fn render_node(
    node: &LayoutNode,
    frame: &mut [u8],
    width: u32,
    height: u32,
    focused_id: &Option<String>,
) {
    if !node.visible {
        return;
    }

    // Dessiner l'arrière-plan
    if node.background_color.a > 0 {
        draw_rect(
            node.rect.x,
            node.rect.y,
            node.rect.width,
            node.rect.height,
            node.background_color,
            frame,
            width,
            height,
        );
    }

    // Vérifier si ce nœud est actuellement en focus
    let is_focused = match focused_id {
        Some(id) => node.id == *id,
        None => false,
    };

    // Dessiner la bordure si nécessaire (avec une couleur spéciale si focusé)
    let border_color = if is_focused {
        // Utiliser une bordure orange vif pour les éléments en focus
        Some(Color::new(255, 140, 0, 255))
    } else {
        node.border_color
    };

    // Dessiner les bordures si nécessaire
    if let Some(border_color) = border_color {
        draw_node_borders(
            node.rect.x,
            node.rect.y,
            node.rect.width,
            node.rect.height,
            is_focused,
            border_color,
            frame,
            width,
            height,
        );
    }

    // Dessiner les enfants
    for child in &node.children {
        render_node(child, frame, width, height, focused_id);
    }
}

/// Dessine les bordures d'un nœud
fn draw_node_borders(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    is_focused: bool,
    color: Color,
    frame: &mut [u8],
    frame_width: u32,
    frame_height: u32,
) {
    // Bordure plus épaisse (3px) pour les éléments en focus
    let border_width = if is_focused { 3 } else { 1 };

    // Haut
    draw_rect(
        x,
        y,
        width,
        border_width,
        color,
        frame,
        frame_width,
        frame_height,
    );

    // Bas
    draw_rect(
        x,
        y + height - border_width,
        width,
        border_width,
        color,
        frame,
        frame_width,
        frame_height,
    );

    // Gauche
    draw_rect(
        x,
        y,
        border_width,
        height,
        color,
        frame,
        frame_width,
        frame_height,
    );

    // Droite
    draw_rect(
        x + width - border_width,
        y,
        border_width,
        height,
        color,
        frame,
        frame_width,
        frame_height,
    );
}
