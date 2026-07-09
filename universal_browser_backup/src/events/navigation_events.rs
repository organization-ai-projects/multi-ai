//! # Gestion des événements de navigation
//!
//! Ce fichier délègue la logique de navigation au `Navigator`.

use crate::state::views::AppView;
use crate::state::{AppState, Navigator};

pub fn navigate_to(state: &mut AppState, target: AppView) {
    let uri = target.uri();
    state.navigator.goto(uri, state).unwrap_or_else(|err| {
        log::error!("Erreur de navigation vers {}: {}", uri, err);
    });
}
