// Composants d'interface utilisateur et état global

use super::primitives::{draw_box, draw_rectangle_outline, URL_BAR_COLOR};
use super::text::draw_text;
use crate::utils::Blinker;
use std::collections::HashSet;

// État global de l'interface
#[derive(Clone)]
pub struct UiState {
    pub current_url: String,
    pub input_active: bool,
    pub rendered_components: HashSet<String>,
    pub cursor_blinker: Blinker,
}

impl UiState {
    pub fn new() -> Self {
        Self {
            current_url: "unl://Home".to_string(),
            input_active: false,
            rendered_components: HashSet::new(),
            cursor_blinker: Blinker::new(500), // Clignoter toutes les 500ms
        }
    }
}

// Fonctions de rendu pour les éléments d'interface courants

/// Dessine la barre d'URL en haut de l'écran
pub fn draw_url_bar(frame: &mut [u8], width: u32, ui_state: &mut UiState) -> u32 {
    // Mettre à jour le clignotement du curseur
    ui_state.cursor_blinker.update();

    let url_bar_height = 30;

    // Variables pour le texte de l'URL
    let url_text_x = 50;
    let url_text_y = 9;
    let url_label_x = 12;
    let url_label_y = url_text_y;

    // Zone de texte de l'URL (champ de saisie)
    let url_field_width = width - 60;
    let url_field_height = 22;
    let url_field_x = url_text_x - 5;
    let url_field_y = 4;

    // Fond pour l'URL - couleur en fonction de l'état d'activation
    let url_background_color = if ui_state.input_active {
        [50, 60, 100, 255] // Bleu plus vif quand actif
    } else {
        [40, 50, 80, 255] // Bleu plus sombre quand inactif
    };

    // 1. Fond de la barre complète
    draw_box(0, 0, width, url_bar_height, frame, width, URL_BAR_COLOR);

    // 2. Zone de texte de l'URL
    draw_box(
        url_field_x,
        url_field_y,
        url_field_width,
        url_field_height,
        frame,
        width,
        url_background_color,
    );

    // 3. Texte "URL:"
    draw_text(
        "URL:",
        url_label_x,
        url_label_y,
        frame,
        width,
        [180, 180, 255, 255],
    );

    // 4. Afficher l'URL actuelle
    draw_text(
        &ui_state.current_url,
        url_text_x,
        url_text_y,
        frame,
        width,
        [255, 255, 255, 255],
    );

    // 5. Indication visuelle d'activation et de l'état actuel
    if ui_state.input_active {
        // Curseur clignotant seulement s'il doit être visible
        if ui_state.cursor_blinker.is_visible() {
            let cursor_pos = url_text_x + ui_state.current_url.len() as u32 * 8;
            draw_text(
                "|",
                cursor_pos,
                url_text_y,
                frame,
                width,
                [255, 255, 100, 255],
            );
        }

        // Longueur actuelle de l'URL (pour aider l'utilisateur)
        let info_text = format!("[{}/50]", ui_state.current_url.len());
        let info_x = width - 100;
        draw_text(
            &info_text,
            info_x,
            url_text_y,
            frame,
            width,
            [200, 200, 255, 255],
        );

        // Instructions
        let help_text_x = width - 300;
        let help_text_y = url_text_y;
        draw_text(
            "Entrée=Valider, Esc=Annuler",
            help_text_x,
            help_text_y,
            frame,
            width,
            [150, 150, 255, 255],
        );
    } else {
        // Instructions
        let help_text_x = width - 200;
        let help_text_y = url_text_y;
        draw_text(
            "[Cliquez pour modifier]",
            help_text_x,
            help_text_y,
            frame,
            width,
            [150, 150, 200, 255],
        );
    }

    // Retourner la hauteur pour savoir où commencer à dessiner le contenu
    url_bar_height
}

/// Dessine un bouton avec texte
pub fn draw_button(label: &str, x: u32, y: u32, frame: &mut [u8], width: u32) -> (u32, u32, u32) {
    let button_width = (label.len() as u32 * 10).max(150);
    let button_height = 50;

    // Couleur du bouton
    let button_color = [50, 100, 200, 255];
    let border_color = [100, 150, 250, 255];

    // Dessiner le fond du bouton
    draw_box(
        x,
        y,
        button_width,
        button_height,
        frame,
        width,
        button_color,
    );

    // Dessiner le contour
    draw_rectangle_outline(
        x,
        y,
        button_width,
        button_height,
        frame,
        width,
        border_color,
    );

    // Dessiner le texte du bouton
    let text_x = x + (button_width - label.len() as u32 * 8) / 2;
    let text_y = y + (button_height - 12) / 2;
    draw_text(label, text_x, text_y, frame, width, [255, 255, 255, 255]);

    // Retourner les dimensions et position
    (button_width, button_height, y + button_height)
}

/// Dessine un texte avec un fond coloré
pub fn draw_text_block(
    text: &str,
    x: u32,
    y: u32,
    frame: &mut [u8],
    width: u32,
) -> (u32, u32, u32) {
    let text_width = text.len() as u32 * 10;
    let text_height = 40;
    let x_pos = x;

    // Fond du texte
    draw_box(
        x_pos - 10,
        y,
        text_width + 20,
        text_height,
        frame,
        width,
        [40, 50, 80, 255],
    );

    // Texte
    let text_x = x_pos + (text_width - text.len() as u32 * 8) / 2;
    let text_y = y + (text_height - 12) / 2;
    draw_text(text, text_x, text_y, frame, width, [200, 200, 255, 255]);

    // Retourner les dimensions et position
    (text_width, text_height, y + text_height)
}
