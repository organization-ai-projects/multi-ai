//! Définition du glyphe pour les caractères inconnus

use crate::engine::render::text::draw::draw_point;
use crate::engine::render::text::font::{CharInfo, Font};

/// Dessine un caractère inconnu (non pris en charge)
pub fn draw_unknown_char(info: CharInfo, frame: &mut [u8], width: u32, font: &Font) {
    let CharInfo { x, y, color, .. } = info;
    let char_width = font.char_width;
    let char_height = font.char_height;

    // Pour les caractères inconnus, dessiner un bloc simple avec une bordure
    for dy in 0..char_height {
        for dx in 0..char_width {
            // Dessiner un rectangle avec les bords
            let is_border = dx == 0 || dx == char_width - 1 || dy == 0 || dy == char_height - 1;
            let is_inner = dx < char_width - 2 && dy < char_height - 2 && dx > 1 && dy > 1;

            if is_border || is_inner {
                let px = x + dx;
                let py = y + dy;
                draw_point(px, py, frame, width, color);
            }
        }
    }
}
