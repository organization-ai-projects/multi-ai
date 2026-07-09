//! # Renderers spécifiques aux vues
//!
//! Ce fichier est responsable de:
//! - Implémenter le rendu visuel de chaque vue
//! - Dessiner les éléments spécifiques à chaque mode d'affichage
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de gestion d'état
//! - Des définitions de layout
//! - Des primitives de dessin génériques
//!
//! Il se concentre sur la représentation visuelle des différentes vues de l'application.

use super::{
    color::Color,
    primitives::{draw_line, draw_rect},
};
use crate::state::{AppState, ViewMode};

/// Effectue le rendu de la vue actuellement active
pub fn render_current_view(state: &AppState, frame: &mut [u8], width: u32, height: u32) {
    match state.current_view {
        ViewMode::Welcome => render_welcome_view(frame, width, height),
        ViewMode::IAExplorer => render_ia_explorer_view(frame, width, height),
        ViewMode::GraphMemory => render_graph_memory_view(frame, width, height),
        ViewMode::FileViewer => render_file_viewer_view(frame, width, height),
    }
}

/// Rendu de la vue d'accueil
fn render_welcome_view(frame: &mut [u8], width: u32, height: u32) {
    // Afficher un texte de bienvenue (simulé avec un rectangle rouge)
    draw_rect(250, 100, 300, 50, Color::red(), frame, width, height);

    // Ajouter quelques lignes décoratives
    draw_line(250, 100, 550, 150, Color::white(), frame, width, height);
    draw_line(250, 150, 550, 100, Color::white(), frame, width, height);
}

/// Rendu de la vue explorateur d'IA
fn render_ia_explorer_view(frame: &mut [u8], width: u32, height: u32) {
    // Dessiner une grille simple avec des lignes
    let grid_x = 250;
    let grid_y = 100;
    let cell_size = 50;
    let grid_width = 4;
    let grid_height = 3;

    // Lignes horizontales
    for i in 0..=grid_height {
        let y = grid_y + i * cell_size;
        draw_line(
            grid_x as i32,
            y as i32,
            (grid_x + grid_width * cell_size) as i32,
            y as i32,
            Color::blue(),
            frame,
            width,
            height,
        );
    }

    // Lignes verticales
    for i in 0..=grid_width {
        let x = grid_x + i * cell_size;
        draw_line(
            x as i32,
            grid_y as i32,
            x as i32,
            (grid_y + grid_height * cell_size) as i32,
            Color::blue(),
            frame,
            width,
            height,
        );
    }
}

/// Rendu de la vue mémoire graphique
fn render_graph_memory_view(frame: &mut [u8], width: u32, height: u32) {
    // Dessiner un graphe simple
    let center_x = 400;
    let center_y = 200;
    let radius = 100;
    let nodes = 5;

    // Points du cercle pour placer les nœuds
    let mut node_positions = Vec::new();
    for i in 0..nodes {
        let angle = 2.0 * std::f32::consts::PI * (i as f32) / (nodes as f32);
        let x = center_x as i32 + (radius as f32 * angle.cos()) as i32;
        let y = center_y as i32 + (radius as f32 * angle.sin()) as i32;
        node_positions.push((x, y));

        // Dessiner le nœud
        draw_rect(
            (x - 5) as u32,
            (y - 5) as u32,
            10,
            10,
            Color::green(),
            frame,
            width,
            height,
        );
    }

    // Connecter tous les nœuds avec des lignes
    for i in 0..nodes {
        for j in i + 1..nodes {
            draw_line(
                node_positions[i].0,
                node_positions[i].1,
                node_positions[j].0,
                node_positions[j].1,
                Color::new(100, 100, 200, 255),
                frame,
                width,
                height,
            );
        }
    }
}

/// Rendu de la vue explorateur de fichiers
fn render_file_viewer_view(frame: &mut [u8], width: u32, height: u32) {
    // Dessiner des lignes pour simuler du texte
    let start_x = 420;
    let start_y = 80;
    let line_height = 20;
    let lines = 15;

    for i in 0..lines {
        let y = start_y + i * line_height;
        let line_length = ((i % 5) + 2) as u32 * 50; // Longueurs variables

        draw_line(
            start_x as i32,
            y as i32,
            (start_x + line_length) as i32,
            y as i32,
            Color::black(),
            frame,
            width,
            height,
        );
    }
}
