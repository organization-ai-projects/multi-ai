//! Définitions des glyphes pour les lettres (A-Z, a-z)

use crate::engine::render::text::draw::draw_point;
use crate::engine::render::text::font::{CharInfo, Font};

/// Dessine une lettre (A-Z, a-z)
pub fn draw_letter(info: CharInfo, frame: &mut [u8], width: u32, font: &Font) {
    let CharInfo { c, x, y, color } = info;
    let char_width = font.char_width;
    let char_height = font.char_height;

    // Dessiner une lettre
    let is_uppercase = c.is_uppercase();
    let letter_idx = if is_uppercase {
        (c as u8 - b'A') as usize
    } else {
        (c as u8 - b'a') as usize
    };

    // Patron de pixels pour chaque lettre
    for dy in 0..char_height {
        for dx in 0..char_width {
            let should_draw = should_draw_letter_pixel(
                letter_idx,
                dx,
                dy,
                char_width,
                char_height,
                is_uppercase,
                c,
            );

            if should_draw {
                let px = x + dx;
                let py = y + dy;
                draw_point(px, py, frame, width, color);
            }
        }
    }
}

/// Détermine si un pixel à la position (dx, dy) doit être dessiné pour une lettre donnée
fn should_draw_letter_pixel(
    letter_idx: usize,
    dx: u32,
    dy: u32,
    char_width: u32,
    char_height: u32,
    is_uppercase: bool,
    c: char,
) -> bool {
    // Variables utiles pour les conditions
    let left_edge = dx == 0;
    let right_edge = dx == char_width - 1;
    let top_edge = dy == 0;
    let bottom_edge = dy == char_height - 1;
    let mid_x = char_width / 2; // Valeur numérique pour la position centrale en x
    let mid_y = char_height / 2; // Valeur numérique pour la position centrale en y

    // Vérifications booléennes pour les positions centrales
    let at_mid_x = dx == mid_x; // Condition booléenne: est-ce que dx est au milieu?
    let at_mid_y = dy == mid_y; // Condition booléenne: est-ce que dy est au milieu?

    let inner_x = dx > 0 && dx < char_width - 1;
    let inner_y = dy > 0 && dy < char_height - 1;

    // Sélection du patron en fonction de la lettre
    match c {
        // Lettres majuscules
        'A' => (left_edge && dy > 0) || (right_edge && dy > 0) || top_edge || at_mid_y,
        'B' => {
            left_edge
                || ((dx == char_width - 2) && inner_y && dy != mid_y)
                || ((top_edge || bottom_edge || at_mid_y) && inner_x)
        }
        'C' => (left_edge && inner_y) || ((top_edge || bottom_edge) && inner_x),
        'D' => {
            left_edge
                || (right_edge && dy > 1 && dy < char_height - 2)
                || ((top_edge || bottom_edge) && inner_x)
        }
        'E' => left_edge || ((top_edge || bottom_edge || at_mid_y) && inner_x),
        'F' => left_edge || ((top_edge || at_mid_y) && inner_x),
        'G' => {
            (left_edge && inner_y)
                || ((top_edge || bottom_edge) && inner_x)
                || (right_edge && dy > mid_y)
                || (dy == mid_y && dx >= mid_x)
        }
        'H' => left_edge || right_edge || at_mid_y,
        'I' => at_mid_x || ((top_edge || bottom_edge) && inner_x),
        'J' => {
            right_edge && dy < char_height - 2
                || (bottom_edge && dx < char_width - 1)
                || (left_edge && dy > 2 * char_height / 3)
        }
        'K' => {
            left_edge
                || (inner_x
                    && ((dy <= mid_y && dx == char_width - 1 - dy * 2 / mid_y)
                        || (dy > mid_y && dx == (dy - mid_y) * 2 / mid_y)))
        }
        'L' => left_edge || (bottom_edge && inner_x),
        'M' => left_edge || right_edge || (dy < mid_y && (dx == dy || dx == char_width - 1 - dy)),
        'N' => left_edge || right_edge || dx == dy,
        'O' => {
            (left_edge && inner_y)
                || (right_edge && inner_y)
                || ((top_edge || bottom_edge) && inner_x)
        }
        'P' => left_edge || (right_edge && dy < mid_y) || ((top_edge || at_mid_y) && inner_x),
        'Q' => {
            (left_edge && inner_y)
                || (right_edge && dy < 2 * char_height / 3)
                || ((top_edge || bottom_edge) && inner_x)
                || (dx > mid_x && dy > 2 * char_height / 3 && dx == dy - char_height / 4)
        }
        'R' => {
            left_edge
                || (right_edge && dy < char_height / 2)
                || ((top_edge || mid_horizontal) && inner_x)
                || (dy > char_height / 2 && dx == dy - char_height / 4)
        }
        'S' => {
            ((top_edge || bottom_edge || mid_horizontal) && inner_x)
                || (left_edge && dy < char_height / 2 && dy > 0)
                || (right_edge && dy > char_height / 2 && dy < char_height - 1)
        }
        'T' => mid_vertical || (top_edge && inner_x),
        'U' => ((left_edge || right_edge) && dy < char_height - 1) || (bottom_edge && inner_x),
        'V' => {
            (dx < char_width / 2 && dy < char_height - 1 - dx) || (dx >= char_width / 2 && dy < dx)
        }
        'W' => {
            (left_edge || right_edge)
                || (mid_vertical && dy > char_height / 2)
                || (dy > char_height / 2
                    && (dx == dy - char_height / 3
                        || dx == char_width - 1 - (dy - char_height / 3)))
        }
        'X' => dx == dy || dx == char_width - 1 - dy,
        'Y' => {
            ((dx == dy || dx == char_width - 1 - dy) && dy < char_height / 2)
                || (mid_vertical && dy >= char_height / 2)
        }
        'Z' => top_edge || bottom_edge || dx == char_width - 1 - dy,

        // Lettres minuscules
        'a' => {
            (dy >= char_height / 2 && ((left_edge || right_edge) && dy < char_height - 1))
                || (dy == char_height / 2 && inner_x)
                || (bottom_edge && inner_x)
        }
        'b' => {
            left_edge
                || (right_edge && dy >= char_height / 2)
                || ((mid_horizontal || bottom_edge) && inner_x)
        }
        'c' => {
            (dy >= char_height / 2 && left_edge && dy < char_height - 1)
                || ((dy == char_height / 2 || bottom_edge) && inner_x)
        }
        'd' => {
            right_edge
                || (left_edge && dy >= char_height / 2 && dy < char_height - 1)
                || ((mid_horizontal || bottom_edge) && inner_x)
        }
        'e' => {
            (left_edge && dy >= char_height / 2 && dy < char_height - 1)
                || (right_edge && dy >= char_height / 2 && dy < char_height - 1)
                || (mid_horizontal && inner_x)
                || (bottom_edge && inner_x)
        }
        'f' => {
            (mid_vertical && dy >= char_height / 5)
                || (dx == char_width / 3 && top_edge)
                || (mid_horizontal && dx >= char_width / 3)
        }
        'g' => {
            (right_edge && dy >= char_height / 2)
                || (left_edge && dy >= char_height / 2 && dy < char_height - 1)
                || (mid_horizontal && inner_x)
                || (bottom_edge && inner_x)
                || (dy == char_height + 2 && inner_x)
        }
        'h' => left_edge || (right_edge && dy >= char_height / 2) || (mid_horizontal && inner_x),
        'i' => (mid_vertical && dy >= char_height / 3) || (mid_vertical && dy < char_height / 6),
        'j' => {
            (dx == 2 * char_width / 3 && dy >= char_height / 3)
                || (dx == 2 * char_width / 3 && dy < char_height / 6)
                || (bottom_edge && dx < 2 * char_width / 3)
                || (left_edge && dy > 2 * char_height / 3 && dy < char_height - 1)
        }
        'k' => {
            left_edge
                || (dx == char_width - 2 - (dy - char_height / 2) && dy >= char_height / 2)
                || (dx == char_width - 2 - (char_height / 2 - dy)
                    && dy < char_height / 2
                    && dy >= char_height / 4)
        }
        'l' => mid_vertical,
        'm' => {
            (dy >= char_height / 2 && (left_edge || dx == char_width / 2 || right_edge))
                || (mid_horizontal && (dx == char_width / 4 || dx == 3 * char_width / 4))
        }
        'n' => (dy >= char_height / 2 && (left_edge || right_edge)) || (mid_horizontal && inner_x),
        'o' => {
            (dy >= char_height / 2 && dy < char_height - 1 && (left_edge || right_edge))
                || ((mid_horizontal || bottom_edge) && inner_x)
        }
        'p' => {
            (left_edge && dy >= char_height / 2)
                || (right_edge && dy >= char_height / 2 && dy < char_height)
                || ((mid_horizontal || dy == char_height) && inner_x)
        }
        'q' => {
            (right_edge && dy >= char_height / 2)
                || (left_edge && dy >= char_height / 2 && dy < char_height - 1)
                || ((mid_horizontal || bottom_edge) && inner_x)
                || (dy == char_height && dx == 3 * char_width / 4)
        }
        'r' => {
            (left_edge && dy >= char_height / 2)
                || (mid_horizontal && dx < char_width / 2)
                || (dx == char_width / 2 && dy > char_height / 2 && dy < 2 * char_height / 3)
        }
        's' => {
            ((at_mid_y || bottom_edge) && inner_x)
                || (left_edge && dy > mid_y && dy < at_mid_y)
                || (right_edge && dy > at_mid_y && dy < char_height - 1)
        }
        't' => {
            (mid_vertical && dy < char_height - 1)
                || (mid_horizontal && dx > char_width / 4)
                || (bottom_edge && dx > mid_vertical)
        }
        'u' => {
            ((left_edge || right_edge) && dy >= char_height / 2 && dy < char_height - 1)
                || (bottom_edge && inner_x)
        }
        'v' => {
            ((left_edge || right_edge) && dy >= char_height / 2 && dy < 3 * char_height / 4)
                || (dx == dy - char_height / 4 && dy >= 3 * char_height / 4)
                || (dx == char_width - 1 - (dy - char_height / 4) && dy >= 3 * char_height / 4)
        }
        'w' => {
            ((left_edge || right_edge || mid_vertical)
                && dy >= char_height / 2
                && dy < 3 * char_height / 4)
                || ((dx == dy - char_height / 4 || dx == char_width - 1 - (dy - char_height / 4))
                    && dy >= 3 * char_height / 4)
        }
        'x' => {
            (dy >= char_height / 2
                && (dx == dy - char_height / 2 || dx == char_width - 1 - (dy - char_height / 2)))
        }
        'y' => {
            ((left_edge || right_edge) && dy >= char_height / 2 && dy < char_height - 1)
                || (bottom_edge && dx < char_width / 2)
                || (dx == char_width - 1 - (dy - char_height) && dy >= char_height)
        }
        'z' => {
            ((dy == char_height / 2 || bottom_edge) && inner_x)
                || (dx == char_width - 1 - (dy - char_height / 2) && dy >= char_height / 2)
        }

        // Fallback pour toute autre lettre
        _ => {
            // Fallback pour toute autre lettre
            let is_alpha = c.is_alphabetic();
            if is_alpha {
                // Forme basique pour toute autre lettre alphabétique
                (left_edge || right_edge) || (at_mid_y && inner_x)
            } else {
                // Rectangle pour les caractères non alphabétiques
                (left_edge || right_edge || top_edge || bottom_edge)
            }
        }
    }
}
