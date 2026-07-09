// Utilitaires généraux pour le navigateur

use std::time::{Duration, Instant};

// Structure pour gérer le clignotement du curseur
#[derive(Clone)]
pub struct Blinker {
    last_toggle: Instant,
    toggle_interval: Duration,
    visible: bool,
}

impl Blinker {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            last_toggle: Instant::now(),
            toggle_interval: Duration::from_millis(interval_ms),
            visible: true,
        }
    }

    pub fn update(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_toggle) >= self.toggle_interval {
            self.visible = !self.visible;
            self.last_toggle = now;
            true
        } else {
            false
        }
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}

// Fonctions pour gérer l'entrée de texte
pub fn sanitize_input(input: &str) -> String {
    // Limiter aux caractères alphanumériques, tirets, points et slash pour les URLs
    input
        .chars()
        .filter(|c| {
            c.is_alphanumeric() || *c == '/' || *c == '.' || *c == '-' || *c == ':' || *c == '_'
        })
        .collect()
}
