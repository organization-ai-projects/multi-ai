//! Définitions des glyphes pour les caractères spéciaux (., :, /, _, -)

use crate::engine::render::text::draw::draw_point;
use crate::engine::render::text::font::{CharInfo, Font};

/// Dessine un caractère spécial (., :, /, _, -)
pub fn draw_special_char(info: CharInfo, frame: &mut [u8], width: u32, font: &Font) {
    let CharInfo { c, x, y, color } = info;
    let char_width = font.char_width;
    let char_height = font.char_height;

    // Dessiner selon le type de caractère spécial
    match c {
        ':' => {
            // Points verticaux
            draw_point(x + char_width / 2, y + char_height / 3, frame, width, color);
            draw_point(
                x + char_width / 2,
                y + 2 * char_height / 3,
                frame,
                width,
                color,
            );
        }
        '/' => {
            // Diagonale
            for dy in 0..char_height {
                let dx = char_width - 1 - dy * char_width / char_height;
                draw_point(x + dx, y + dy, frame, width, color);
            }
        }
        '.' => {
            // Point en bas
            draw_point(
                x + char_width / 2,
                y + 2 * char_height / 3,
                frame,
                width,
                color,
            );
        }
        '_' => {
            // Ligne horizontale en bas
            for dx in 0..char_width {
                draw_point(x + dx, y + char_height - 1, frame, width, color);
            }
        }
        '-' => {
            // Ligne horizontale au milieu
            for dx in 0..char_width {
                draw_point(x + dx, y + char_height / 2, frame, width, color);
            }
        }
        _ => {}
    }
}
