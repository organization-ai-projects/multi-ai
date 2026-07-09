//! # Point d'entrée de l'application Universal Browser
//!
//! Ce fichier est responsable de:
//! - Initialiser le système de journalisation
//! - Démarrer l'application de manière asynchrone
//!
//! Ce fichier NE DOIT PAS contenir:
//! - De la logique métier
//! - Des configurations d'UI
//! - Des gestionnaires d'événements
//!
//! Il sert uniquement de point d'entrée et délègue tout à `app.rs`.

mod app;
mod events_main; // Renommé depuis `events`
mod html_engine;
mod layout;
mod navigation;
mod plugins;
mod renderer;
mod state;
mod ws_bridge;
mod events;

use app::run;

fn main() {
    env_logger::init();
    pollster::block_on(run());
}
