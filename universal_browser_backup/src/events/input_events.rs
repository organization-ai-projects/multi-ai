//! # Gestion des entrées utilisateur
//!
//! Ce fichier est responsable de:
//! - Gérer les événements de clavier et de souris
//! - Mettre à jour l'état de l'application en fonction des entrées utilisateur
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de navigation
//! - Des gestionnaires d'événements de fenêtre
//!
//! Il se concentre uniquement sur les événements d'entrée utilisateur.

use crate::events::{navigate_to, NavigationTarget};
use crate::state::{views::AppView, AppState};
use winit::event::{ElementState, KeyboardInput, MouseButton, VirtualKeyCode};

pub fn handle_input_event(button_state: ElementState, button: MouseButton, state: &mut AppState) {
    if button == MouseButton::Left {
        state.handle_mouse_press(button_state == ElementState::Pressed);

        if button_state == ElementState::Released {
            if let Some(node) = state
                .layout_manager
                .root
                .find_node_at(state.mouse_x, state.mouse_y)
            {
                match node.id.as_str() {
                    "option_welcome" => navigate_to(state, NavigationTarget::Welcome),
                    "option_ia" => navigate_to(state, NavigationTarget::IAExplorer),
                    "option_graph" => navigate_to(state, NavigationTarget::GraphMemory),
                    "option_files" => navigate_to(state, NavigationTarget::FileExplorer),
                    _ => {}
                }
            }
        }
    }
}

pub fn handle_keyboard_input(key: VirtualKeyCode, state: &mut AppState) {
    if let Some(view) = map_key_to_view(key) {
        navigate_to(state, view);
    }
}

fn map_key_to_view(key: VirtualKeyCode) -> Option<AppView> {
    match key {
        VirtualKeyCode::Key1 => Some(AppView::Welcome),
        VirtualKeyCode::Key2 => Some(AppView::IAExplorer),
        VirtualKeyCode::Key3 => Some(AppView::GraphMemory),
        VirtualKeyCode::Key4 => Some(AppView::FileViewer),
        _ => None,
    }
}
