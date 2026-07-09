//! # Module des événements
//!
//! Ce module est responsable de:
//! - Gérer les événements de fenêtre
//! - Gérer les entrées utilisateur (clavier, souris)
//! - Gérer les événements de navigation
//! - Gérer les événements liés aux plugins
//!
//! Il coordonne les sous-modules pour traiter les événements de manière modulaire.

mod input_events;
mod navigation_events;
mod plugin_events;
mod window_events;

pub use input_events::{handle_input_event, handle_keyboard_input};
pub use navigation_events::{navigate_to, NavigationTarget};
pub use plugin_events::activate_plugin;
pub use window_events::handle_window_event;
