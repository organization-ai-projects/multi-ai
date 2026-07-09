//! # Gestion des événements de fenêtre
//!
//! Ce fichier est responsable de:
//! - Gérer les événements liés à la fenêtre (fermeture, redimensionnement, etc.)
//! - Mettre à jour l'état de l'application en fonction des événements de fenêtre
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de navigation
//! - Des gestionnaires d'entrées utilisateur
//!
//! Il se concentre uniquement sur les événements de fenêtre.

use crate::renderer::Renderer;
use crate::state::AppState;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;

pub fn handle_window_event(
    event: WindowEvent,
    control_flow: &mut ControlFlow,
    state: &mut AppState,
    renderer: &mut Renderer,
) {
    match event {
        WindowEvent::CloseRequested => {
            *control_flow = ControlFlow::Exit;
        }
        WindowEvent::Resized(size) => {
            state.resize(size.width, size.height);
            renderer.resize(size.width, size.height);
        }
        WindowEvent::CursorMoved { position, .. } => {
            state.update_mouse(position.x as u32, position.y as u32);
        }
        _ => {}
    }
}
