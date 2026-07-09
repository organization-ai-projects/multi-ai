//! Fonctions principales pour dessiner du texte

use super::font::{CharInfo, Font};
use super::glyphs::{
    digits::draw_digit, letters::draw_letter, specials::draw_special_char,
    unknown::draw_unknown_char,
};

/// Dessine du texte avec un rendu pixel par pixel
pub fn draw_text(text: &str, x: u32, y: u32, frame: &mut [u8], width: u32, color: [u8; 4]) {
    let font = Font::default();
    let mut current_x = x;

    for c in text.chars() {
        let char_info = CharInfo {
            c,
            x: current_x,
            y,
            color,
        };

        match c {
            'A'..='Z' | 'a'..='z' => draw_letter(char_info, frame, width, &font),
            '0'..='9' => draw_digit(char_info, frame, width, &font),
            ':' | '/' | '.' | '_' | '-' => draw_special_char(char_info, frame, width, &font),
            ' ' => {} // Espace - ne rien dessiner
            _ => draw_unknown_char(char_info, frame, width, &font),
        }

        // Avancer à la position du prochain caractère
        current_x += font.char_width + font.spacing;
    }
}

/// Fonction utilitaire pour dessiner un point unique
pub fn draw_point(x: u32, y: u32, frame: &mut [u8], width: u32, color: [u8; 4]) {
    let idx = ((y * width + x) * 4) as usize;
    if idx + 3 < frame.len() {
        frame[idx..idx + 4].copy_from_slice(&color);
    }
}
