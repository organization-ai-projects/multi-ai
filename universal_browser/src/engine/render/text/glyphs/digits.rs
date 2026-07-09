//! Définitions des glyphes pour les chiffres (0-9)

use crate::engine::render::text::draw::draw_point;
use crate::engine::render::text::font::{CharInfo, Font};

/// Dessine un chiffre (0-9)
pub fn draw_digit(info: CharInfo, frame: &mut [u8], width: u32, font: &Font) {
    let CharInfo { c, x, y, color } = info;
    let char_width = font.char_width;
    let char_height = font.char_height;

    // Convertir le caractère en chiffre
    let digit = c as u8 - b'0';

    // Patron de pixels pour chaque chiffre
    for dy in 0..char_height {
        for dx in 0..char_width {
            let should_draw = should_draw_digit_pixel(digit, dx, dy, char_width, char_height);

            if should_draw {
                let px = x + dx;
                let py = y + dy;
                draw_point(px, py, frame, width, color);
            }
        }
    }
}

/// Détermine si un pixel à la position (dx, dy) doit être dessiné pour un chiffre donné
fn should_draw_digit_pixel(digit: u8, dx: u32, dy: u32, char_width: u32, char_height: u32) -> bool {
    // Variables utiles pour les conditions
    let left_edge = dx == 0;
    let right_edge = dx == char_width - 1;
    let top_edge = dy == 0;
    let bottom_edge = dy == char_height - 1;
    let mid_horizontal = dy == char_height / 2;

    let inner_x = dx > 0 && dx < char_width - 1;
    let inner_y = dy > 0 && dy < char_height - 1;

    match digit {
        // 0 - cercle avec diagonale
        0 => {
            (left_edge && inner_y)
                || (right_edge && inner_y)
                || (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (dx == dy || dx == char_width - 1 - dy) // Diagonales pour distinguer du O
        }
        // 1 - barre verticale avec base et toit
        1 => {
            (dx == char_width / 2)
                || (bottom_edge && inner_x)
                || (dx == char_width / 2 - 1 && dy <= 2)
        }
        // 2 - forme de S horizontale
        2 => {
            (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (mid_horizontal && inner_x)
                || (right_edge && dy < char_height / 2 && dy > 0)
                || (left_edge && dy > char_height / 2 && dy < char_height - 1)
        }
        // 3 - forme E sans barre gauche
        3 => {
            (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (mid_horizontal && inner_x)
                || (right_edge && inner_y)
        }
        // 4 - forme de h avec barre du milieu complète
        4 => {
            (left_edge && dy < char_height / 2)
                || (right_edge && inner_y)
                || (mid_horizontal && inner_x)
        }
        // 5 - forme de S horizontale inversée
        5 => {
            (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (mid_horizontal && inner_x)
                || (left_edge && dy < char_height / 2 && dy > 0)
                || (right_edge && dy > char_height / 2 && dy < char_height - 1)
        }
        // 6 - forme de 9 inversé
        6 => {
            (left_edge && inner_y)
                || (right_edge && dy > char_height / 2 && dy < char_height - 1)
                || (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (mid_horizontal && inner_x)
        }
        // 7 - barre horizontale en haut et barre diagonale
        7 => (top_edge) || (dx == char_width - 1 - dy / 2 && dy < char_height - 1),
        // 8 - deux cercles superposés
        8 => {
            (left_edge && inner_y)
                || (right_edge && inner_y)
                || (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (mid_horizontal && inner_x)
        }
        // 9 - forme de 6 inversé
        9 => {
            (right_edge && inner_y)
                || (left_edge && dy < char_height / 2 && dy > 0)
                || (top_edge && inner_x)
                || (bottom_edge && inner_x)
                || (mid_horizontal && inner_x)
        }
        // Fallback pour tout autre chiffre (ne devrait pas arriver)
        _ => false,
    }
}
