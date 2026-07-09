use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Commandes vectorielles (type SVG-like)
#[derive(Clone, Serialize, Deserialize, Debug, bincode_next::Encode, bincode_next::Decode)]
pub enum VectorCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    CurveTo(f32, f32, f32, f32),
}

/// Données vectorielles d'un glyphe
#[derive(Clone, Serialize, Deserialize, Debug, bincode_next::Encode, bincode_next::Decode)]
pub struct GlyphPath {
    pub commands: Vec<VectorCommand>,
}

/// Sortie typographique complète : pixel + vecteur
#[derive(Clone, Serialize, Deserialize, Debug, bincode_next::Encode, bincode_next::Decode)]
pub struct GlyphOutput {
    pub bitmap: Vec<bool>,
    pub vector: GlyphPath,
    pub width: usize,
    pub height: usize,
}

/// Mémoire persistante de tous les glyphes connus
#[derive(Serialize, Deserialize, Default, bincode_next::Encode, bincode_next::Decode)]
pub struct GlyphMemory {
    pub data: HashMap<(char, String), GlyphOutput>,
}

/// IA complète
pub struct GlyphIA {
    pub memory: GlyphMemory,
}

impl GlyphIA {
    pub fn new() -> Self {
        Self {
            memory: GlyphMemory::default(),
        }
    }

    /// Génère (ou charge) un glyphe
    pub fn generate(&mut self, c: char, font: &str) -> &GlyphOutput {
        self.memory
            .data
            .entry((c, font.to_string()))
            .or_insert_with(|| {
                let (bitmap, vector) = Self::generate_dual(c, font);
                GlyphOutput {
                    bitmap,
                    vector,
                    width: 8,
                    height: 12,
                }
            })
    }

    /// Génère à la fois le bitmap ET le vecteur
    fn generate_dual(c: char, font: &str) -> (Vec<bool>, GlyphPath) {
        let width = 8;
        let height = 12;
        let mut bitmap = vec![false; width * height];
        let mut commands = vec![];

        match c.to_ascii_uppercase() {
            'A' => {
                for y in 0..height {
                    let left = width / 2 - y / 2;
                    let right = width / 2 + y / 2;
                    if left < right && left < width && right < width {
                        bitmap[y * width + left] = true;
                        bitmap[y * width + right] = true;
                    }
                    if y == height / 2 {
                        for x in left..=right {
                            if x < width {
                                bitmap[y * width + x] = true;
                            }
                        }
                    }
                }
                commands.push(VectorCommand::MoveTo(0.0, 12.0));
                commands.push(VectorCommand::LineTo(4.0, 0.0));
                commands.push(VectorCommand::LineTo(8.0, 12.0));
                commands.push(VectorCommand::MoveTo(2.0, 6.0));
                commands.push(VectorCommand::LineTo(6.0, 6.0));
            }
            'B' => {
                // Ligne verticale à gauche
                for y in 0..height {
                    bitmap[y * width] = true;
                }
                // Lignes horizontales (haut, milieu, bas)
                for x in 0..width - 1 {
                    bitmap[x] = true;
                    bitmap[height / 2 * width + x] = true;
                    bitmap[(height - 1) * width + x] = true;
                }
                // Ligne verticale à droite (haut et bas)
                for y in 1..height / 2 {
                    bitmap[y * width + width - 1] = true;
                }
                for y in height / 2 + 1..height - 1 {
                    bitmap[y * width + width - 1] = true;
                }

                commands.push(VectorCommand::MoveTo(0.0, 0.0));
                commands.push(VectorCommand::LineTo(6.0, 0.0));
                commands.push(VectorCommand::LineTo(8.0, 2.0));
                commands.push(VectorCommand::LineTo(8.0, 5.0));
                commands.push(VectorCommand::LineTo(6.0, 6.0));
                commands.push(VectorCommand::LineTo(0.0, 6.0));
                commands.push(VectorCommand::MoveTo(0.0, 6.0));
                commands.push(VectorCommand::LineTo(6.0, 6.0));
                commands.push(VectorCommand::LineTo(8.0, 8.0));
                commands.push(VectorCommand::LineTo(8.0, 10.0));
                commands.push(VectorCommand::LineTo(6.0, 12.0));
                commands.push(VectorCommand::LineTo(0.0, 12.0));
            }
            '1' => {
                // Ligne verticale au centre
                for y in 0..height {
                    bitmap[y * width + width / 2] = true;
                }
                // Ligne horizontale en bas
                for x in 0..width {
                    bitmap[(height - 1) * width + x] = true;
                }
                // Petit trait en haut à gauche
                bitmap[1 * width + width / 2 - 1] = true;
                bitmap[0 * width + width / 2 - 1] = true;

                commands.push(VectorCommand::MoveTo(2.0, 2.0));
                commands.push(VectorCommand::LineTo(4.0, 0.0));
                commands.push(VectorCommand::LineTo(4.0, 12.0));
                commands.push(VectorCommand::MoveTo(2.0, 12.0));
                commands.push(VectorCommand::LineTo(6.0, 12.0));
            }
            '.' => {
                // Point en bas
                bitmap[(height - 2) * width + width / 2] = true;
                bitmap[(height - 3) * width + width / 2] = true;
                bitmap[(height - 2) * width + width / 2 - 1] = true;
                bitmap[(height - 2) * width + width / 2 + 1] = true;

                commands.push(VectorCommand::MoveTo(3.0, 10.0));
                commands.push(VectorCommand::LineTo(5.0, 10.0));
                commands.push(VectorCommand::LineTo(5.0, 12.0));
                commands.push(VectorCommand::LineTo(3.0, 12.0));
                commands.push(VectorCommand::LineTo(3.0, 10.0));
            }
            '/' => {
                // Diagonale
                for y in 0..height {
                    let x = width - 1 - y * width / height;
                    if x < width {
                        bitmap[y * width + x] = true;
                    }
                }

                commands.push(VectorCommand::MoveTo(7.0, 0.0));
                commands.push(VectorCommand::LineTo(1.0, 12.0));
            }
            ':' => {
                // Deux points verticaux
                bitmap[(height / 3) * width + width / 2] = true;
                bitmap[(height / 3) * width + width / 2 - 1] = true;
                bitmap[(height / 3) * width + width / 2 + 1] = true;
                bitmap[(height / 3 - 1) * width + width / 2] = true;

                bitmap[(2 * height / 3) * width + width / 2] = true;
                bitmap[(2 * height / 3) * width + width / 2 - 1] = true;
                bitmap[(2 * height / 3) * width + width / 2 + 1] = true;
                bitmap[(2 * height / 3 + 1) * width + width / 2] = true;

                commands.push(VectorCommand::MoveTo(3.0, 3.0));
                commands.push(VectorCommand::LineTo(5.0, 3.0));
                commands.push(VectorCommand::LineTo(5.0, 5.0));
                commands.push(VectorCommand::LineTo(3.0, 5.0));
                commands.push(VectorCommand::LineTo(3.0, 3.0));

                commands.push(VectorCommand::MoveTo(3.0, 8.0));
                commands.push(VectorCommand::LineTo(5.0, 8.0));
                commands.push(VectorCommand::LineTo(5.0, 10.0));
                commands.push(VectorCommand::LineTo(3.0, 10.0));
                commands.push(VectorCommand::LineTo(3.0, 8.0));
            }
            _ => {
                // Pour tous les autres caractères, forme simple
                for y in 0..height {
                    for x in 0..width {
                        if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                            bitmap[y * width + x] = true;
                        }
                    }
                }
                commands.push(VectorCommand::MoveTo(0.0, 0.0));
                commands.push(VectorCommand::LineTo(0.0, 12.0));
                commands.push(VectorCommand::LineTo(8.0, 12.0));
                commands.push(VectorCommand::LineTo(8.0, 0.0));
                commands.push(VectorCommand::LineTo(0.0, 0.0));
            }
        }

        (bitmap, GlyphPath { commands })
    }

    /// Sauvegarde la mémoire `.ron`
    pub fn save_to_ron(&self, path: &str) {
        if let Ok(serialized) = ron::to_string(&self.memory) {
            std::fs::write(path, serialized).ok();
        }
    }

    /// Recharge depuis un `.ron`
    pub fn load_from_ron(path: &str) -> Self {
        if let Ok(data) = std::fs::read_to_string(path) {
            if let Ok(mem) = ron::from_str(&data) {
                return Self { memory: mem };
            }
        }
        Self::new()
    }

    /// Sauvegarde mémoire `.bin` (plus compact)
    pub fn save_to_bin(&self, path: &str) {
        if let Ok(serialized) = bincode_next::encode_to_vec(&self.memory, bincode_next::config::standard()) {
            std::fs::write(path, serialized).ok();
        }
    }

    /// Recharge `.bin`
    pub fn load_from_bin(path: &str) -> Self {
        if let Ok(data) = std::fs::read(path) {
            if let Ok(mem) = bincode_next::decode_from_slice(&data[..], bincode_next::config::standard()).map(|(v, _)| v) {
                return Self { memory: mem };
            }
        }
        Self::new()
    }
}
