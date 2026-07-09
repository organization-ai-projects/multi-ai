//! # Gestionnaire principal des événements
//!
//! Ce fichier coordonne les sous-modules d'événements.

use crate::events::{
    activate_plugin, handle_input_event, handle_keyboard_input, handle_window_event, navigate_to,
    NavigationTarget,
};
use crate::renderer::Renderer;
use crate::state::AppState;
use std::any::Any;
use winit::event::{DeviceEvent, ElementState, Event, KeyboardInput, MouseButton, WindowEvent};
use winit::event_loop::ControlFlow;

/// Gère tous les événements de l'application
pub fn handle_event<T: 'static + Any>(
    event: Event<T>,
    control_flow: &mut ControlFlow,
    state: &mut AppState,
    renderer: &mut Renderer,
) {
    match event {
        // Événements liés à la fenêtre
        Event::WindowEvent { event, .. } => {
            handle_window_event(event, control_flow, state, renderer);
        }

        // Événements liés aux périphériques (clavier, souris, etc.)
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::Key(keyboard_input) => {
                if let Some(key) = keyboard_input.virtual_keycode {
                    handle_keyboard_input(key, state);
                }
            }
            DeviceEvent::Button {
                button,
                state: button_state,
            } => {
                if button == 1 {
                    handle_input_event(
                        if button_state == ElementState::Pressed {
                            ElementState::Pressed
                        } else {
                            ElementState::Released
                        },
                        MouseButton::Left,
                        state,
                    );
                }
            }
            _ => {}
        },

        // Événements de rendu
        Event::MainEventsCleared => {
            if state.redraw_requested {
                renderer.render(state).unwrap();
                state.redraw_requested = false;
            }
        }

        // Événements utilisateur personnalisés
        Event::UserEvent(user_event) => {
            handle_user_event(user_event, state);
        }

        // Autres événements
        _ => {}
    }
}

/// Gère les événements utilisateur personnalisés
fn handle_user_event<T: 'static + Any>(user_event: T, state: &mut AppState) {
    if let Some(target) = parse_user_event(&user_event) {
        navigate_to(state, target);
    } else if let Some(plugin_id) = parse_plugin_event(&user_event) {
        activate_plugin(&plugin_id, state).unwrap_or_else(|err| {
            log::error!(
                "Erreur lors de l'activation du plugin {}: {}",
                plugin_id,
                err
            );
        });
    }
}

/// Parse un événement utilisateur en NavigationTarget
fn parse_user_event<T: 'static + std::any::Any>(event: &T) -> Option<NavigationTarget> {
    if let Some(event_str) = event.downcast_ref::<String>() {
        match event_str.as_str() {
            "welcome" => Some(NavigationTarget::Welcome),
            "ia_explorer" => Some(NavigationTarget::IAExplorer),
            "graph_memory" => Some(NavigationTarget::GraphMemory),
            "file_explorer" => Some(NavigationTarget::FileExplorer), // Correction ici
            _ => None,
        }
    } else {
        None
    }
}

/// Parse un événement utilisateur en ID de plugin
fn parse_plugin_event<T: 'static + std::any::Any>(event: &T) -> Option<String> {
    if let Some(event_str) = event.downcast_ref::<String>() {
        if event_str.starts_with("plugin:") {
            Some(event_str.trim_start_matches("plugin:").to_string())
        } else {
            None
        }
    } else {
        None
    }
}
