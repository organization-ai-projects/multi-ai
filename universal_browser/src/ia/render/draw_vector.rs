use crate::ia::glyph::ai::{GlyphPath, VectorCommand};

pub fn draw_vector(
    x: f32,
    y: f32,
    path: &GlyphPath,
    frame: &mut [u8],
    frame_width: usize,
    scale: f32,
) {
    // Position courante pour les tracés
    let mut current_x = x;
    let mut current_y = y;

    for cmd in &path.commands {
        match cmd {
            VectorCommand::MoveTo(nx, ny) => {
                // Déplacer sans tracer
                current_x = x + nx * scale;
                current_y = y + ny * scale;
            }
            VectorCommand::LineTo(nx, ny) => {
                // Tracer une ligne du point actuel au nouveau point
                let target_x = x + nx * scale;
                let target_y = y + ny * scale;

                // Algorithme de Bresenham pour tracer une ligne
                let dx = (target_x - current_x).abs();
                let dy = (target_y - current_y).abs();

                let sx = if current_x < target_x { 1.0 } else { -1.0 };
                let sy = if current_y < target_y { 1.0 } else { -1.0 };

                let mut err = dx - dy;

                let mut cx = current_x;
                let mut cy = current_y;

                while (cx - target_x).abs() > 0.5 || (cy - target_y).abs() > 0.5 {
                    // Dessiner le pixel actuel
                    let pixel_x = cx as usize;
                    let pixel_y = cy as usize;
                    let idx = (pixel_y * frame_width + pixel_x) * 4;

                    if idx + 3 < frame.len() {
                        frame[idx..idx + 4].copy_from_slice(&[220, 220, 255, 255]);
                    }

                    let e2 = 2.0 * err;
                    if e2 > -dy {
                        err -= dy;
                        cx += sx;
                    }
                    if e2 < dx {
                        err += dx;
                        cy += sy;
                    }
                }

                current_x = target_x;
                current_y = target_y;
            }
            VectorCommand::CurveTo(c1x, c1y, c2x, c2y) => {
                // Calcul des positions absolues des points de contrôle
                let p0x = current_x;
                let p0y = current_y;
                let p1x = x + c1x * scale;
                let p1y = y + c1y * scale;
                let p2x = x + c2x * scale;
                let p2y = y + c2y * scale;

                // Utiliser une vraie courbe de Bézier cubique
                // Formule: B(t) = (1-t)³P₀ + 3(1-t)²tP₁ + 3(1-t)t²P₂ + t³P₃
                // Où P₀ est le point actuel, P₁ et P₂ sont les points de contrôle, P₃ est le point cible

                // Nombre de segments pour approximer la courbe (plus élevé = plus lisse)
                let steps = 20;

                let mut last_x = p0x;
                let mut last_y = p0y;

                for i in 1..=steps {
                    let t = i as f32 / steps as f32;
                    let t2 = t * t;
                    let t3 = t2 * t;
                    let mt = 1.0 - t;
                    let mt2 = mt * mt;
                    let mt3 = mt2 * mt;

                    // Courbe de Bézier cubique
                    let px = mt3 * p0x + 3.0 * mt2 * t * p1x + 3.0 * mt * t2 * p2x + t3 * p2x;
                    let py = mt3 * p0y + 3.0 * mt2 * t * p1y + 3.0 * mt * t2 * p2y + t3 * p2y;

                    // Dessiner un segment entre le dernier point et le point actuel
                    draw_line_segment(last_x, last_y, px, py, frame, frame_width);

                    last_x = px;
                    last_y = py;
                }

                // Mettre à jour la position courante
                current_x = p2x;
                current_y = p2y;
            }
        }
    }
}

// Fonction auxiliaire pour dessiner un segment de ligne
fn draw_line_segment(x1: f32, y1: f32, x2: f32, y2: f32, frame: &mut [u8], frame_width: usize) {
    // Algorithme de Bresenham pour tracer une ligne
    let dx = (x2 - x1).abs();
    let dy = (y2 - y1).abs();

    let sx = if x1 < x2 { 1.0 } else { -1.0 };
    let sy = if y1 < y2 { 1.0 } else { -1.0 };

    let mut err = dx - dy;

    let mut cx = x1;
    let mut cy = y1;

    while (cx - x2).abs() > 0.5 || (cy - y2).abs() > 0.5 {
        // Dessiner le pixel actuel
        let pixel_x = cx as usize;
        let pixel_y = cy as usize;
        let idx = (pixel_y * frame_width + pixel_x) * 4;

        if idx + 3 < frame.len() {
            // Couleur légèrement différente pour les courbes
            frame[idx..idx + 4].copy_from_slice(&[180, 190, 255, 255]);
        }

        let e2 = 2.0 * err;
        if e2 > -dy {
            err -= dy;
            cx += sx;
        }
        if e2 < dx {
            err += dx;
            cy += sy;
        }
    }
}
