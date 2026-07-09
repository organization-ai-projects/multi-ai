//! # Module d'initialisation et configuration de l'application
//!
//! Ce fichier est responsable de:
//! - Créer et configurer la fenêtre principale
//! - Initialiser les gestionnaires principaux (état, rendu)
//! - Démarrer la boucle d'événements principale
//!
//! Ce fichier NE DOIT PAS contenir:
//! - La logique de gestion des événements (déléguée à events.rs)
//! - La définition du layout (déléguée à layout/presets.rs)
//! - Le code de rendu (délégué à renderer/)
//!
//! Ce module orchestre uniquement les composants majeurs sans implémenter leurs détails.

use crate::{
    events_main::handle_event, // Renommé depuis `events`
    layout::create_default_layout,
    navigation::presets::goto_welcome,
    renderer::Renderer,
    state::AppState,
};
use winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};

pub async fn run() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Universal Browser")
        .with_inner_size(LogicalSize::new(1280.0, 720.0))
        .build(&event_loop)
        .expect("Failed to create window");

    let mut state = AppState::new(window.inner_size());

    // Initialiser les plugins intégrés
    state
        .plugin_manager
        .load_builtin_plugins(&mut state)
        .expect("Failed to load builtin plugins");

    // Essayer de se connecter au WebSocket
    match state.ws_bridge.connect() {
        Ok(_) => log::info!("Connected to WebSocket server"),
        Err(e) => log::warn!("Failed to connect to WebSocket server: {}", e),
    }

    let mut renderer = Renderer::new(&window).expect("Failed to create renderer");

    // Configuration du layout initial
    create_default_layout(&mut state);

    // Naviguer vers la page d'accueil
    goto_welcome(&mut state.navigator, &mut state).expect("Failed to navigate to welcome page");

    // Définir la valeur initiale de redraw_requested
    state.redraw_requested = true;

    // Cette méthode ne retourne jamais
    event_loop.run(move |event, _, control_flow| {
        // Mettre à jour le bridge WebSocket
        let ws_events = state.ws_bridge.update();

        // Transmettre les événements WebSocket aux plugins
        for event in ws_events {
            state.plugin_manager.handle_event(event, &mut state);
        }

        // Mettre à jour tous les plugins
        state.plugin_manager.update_all(&mut state);

        // Gérer les événements standard
        handle_event(event, control_flow, &mut state, &mut renderer)
    })
}
