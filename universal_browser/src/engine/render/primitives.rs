// Fonctions pour dessiner des formes géométriques de base

/// Dessine un rectangle plein
pub fn draw_box(x: u32, y: u32, w: u32, h: u32, frame: &mut [u8], width: u32, color: [u8; 4]) {
    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            let idx = ((py * width + px) * 4) as usize;
            if idx + 3 < frame.len() {
                frame[idx..idx + 4].copy_from_slice(&color);
            }
        }
    }
}

/// Dessine uniquement le contour d'un rectangle
pub fn draw_rectangle_outline(
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    frame: &mut [u8],
    width: u32,
    color: [u8; 4],
) {
    // Lignes horizontales (haut et bas)
    for dx in 0..w {
        // Ligne du haut
        let idx_top = ((y * width + (x + dx)) * 4) as usize;
        if idx_top + 3 < frame.len() {
            frame[idx_top..idx_top + 4].copy_from_slice(&color);
        }

        // Ligne du bas
        let idx_bottom = (((y + h - 1) * width + (x + dx)) * 4) as usize;
        if idx_bottom + 3 < frame.len() {
            frame[idx_bottom..idx_bottom + 4].copy_from_slice(&color);
        }
    }

    // Lignes verticales (gauche et droite)
    for dy in 0..h {
        // Ligne de gauche
        let idx_left = (((y + dy) * width + x) * 4) as usize;
        if idx_left + 3 < frame.len() {
            frame[idx_left..idx_left + 4].copy_from_slice(&color);
        }

        // Ligne de droite
        let idx_right = (((y + dy) * width + (x + w - 1)) * 4) as usize;
        if idx_right + 3 < frame.len() {
            frame[idx_right..idx_right + 4].copy_from_slice(&color);
        }
    }
}

// Couleurs standard
pub const BACKGROUND_COLOR: [u8; 4] = [20, 30, 50, 255];
pub const URL_BAR_COLOR: [u8; 4] = [30, 40, 60, 255];
