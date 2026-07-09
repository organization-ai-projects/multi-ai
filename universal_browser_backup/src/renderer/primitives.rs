//! # Primitives de dessin
//! 
//! Ce fichier est responsable de:
//! - Définir les fonctions de dessin de bas niveau
//! - Implémenter des algorithmes efficaces pour le dessin de formes
//! - Manipuler directement les pixels du frame buffer
//! 
//! Ce fichier NE DOIT PAS contenir:
//! - Des logiques de layout
//! - Des définitions de composants d'interface
//! - Des gestionnaires d'événements
//! 
//! Il se concentre uniquement sur le dessin de formes élémentaires au niveau des pixels.

// Fonctions de dessin de formes primitives
use super::color::Color;

// Ces fonctions sont définies ici pour éviter de les mettre toutes dans core.rs
pub(crate) fn draw_rect(
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: Color,
    frame: &mut [u8],
    frame_width: u32,
    frame_height: u32,
) {
    for cy in y..y.saturating_add(height) {
        for cx in x..x.saturating_add(width) {
            if cx < frame_width && cy < frame_height {
                let idx = ((cy * frame_width + cx) * 4) as usize;
                if idx + 3 < frame.len() {
                    frame[idx] = color.r;
                    frame[idx + 1] = color.g;
                    frame[idx + 2] = color.b;
                    frame[idx + 3] = color.a;
                }
            }
        }
    }
}

pub(crate) fn draw_line(
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    color: Color,
    frame: &mut [u8],
    frame_width: u32,
    frame_height: u32,
) {
    let frame_width = frame_width as i32;
    let frame_height = frame_height as i32;

    let mut x0 = x0;
    let mut y0 = y0;
    let mut x1 = x1;
    let mut y1 = y1;

    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();
    let mut err = dx / 2;
    let mut y = y0;
    let ystep = if y0 < y1 { 1 } else { -1 };

    for x in x0..=x1 {
        let (px, py) = if steep { (y, x) } else { (x, y) };

        if px >= 0 && px < frame_width && py >= 0 && py < frame_height {
            let idx = ((py as u32 * frame_width as u32 + px as u32) * 4) as usize;
            if idx + 3 < frame.len() {
                frame[idx] = color.r;
                frame[idx + 1] = color.g;
                frame[idx + 2] = color.b;
                frame[idx + 3] = color.a;
            }
        }

        err -= dy;
        if err < 0 {
            y += ystep;
            err += dx;
        }
    }
}
