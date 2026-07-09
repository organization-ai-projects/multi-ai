use crate::memory::{GlyphKnowledge, GlyphVariant, Memory};

// Gestionnaire responsable des opérations sur les glyphes
pub struct GlyphManager<'a> {
    memory: &'a mut Memory,
}

impl<'a> GlyphManager<'a> {
    pub fn new(memory: &'a mut Memory) -> Self {
        Self { memory }
    }

    // Génère des variantes pour un glyphe donné
    pub fn generate_variants(&mut self, letter: char, count: usize) {
        // D'abord, générer la variante standard
        self.generate_glyph_variant(letter);
        println!("Variante 0: Standard (originale)");

        // Ensuite, générer des variantes modifiées
        for i in 1..count {
            let (vector, bitmap, description) = match i % 4 {
                0 => (
                    // Variante originale
                    self.generate_vector_glyph(letter),
                    self.generate_bitmap_glyph(letter),
                    "Standard (originale)".to_string(),
                ),
                1 => {
                    // Variante étroite
                    let v = self
                        .generate_vector_glyph(letter)
                        .replace("10,0", "7,0")
                        .replace("5,10", "4,10")
                        .replace("7.5,5", "5.5,5")
                        .replace("2.5,5", "2,5");

                    // Bitmap étroit pour le A (exemple concret)
                    let b = match letter {
                        'A' => "bitmap:5x7:02040911110909".to_string(),
                        _ => "bitmap:5x7:02040911110909".to_string(), // Par défaut utiliser le A étroit
                    };

                    (v, b, "Version étroite".to_string())
                }
                2 => {
                    // Variante large
                    let v = self
                        .generate_vector_glyph(letter)
                        .replace("10,0", "14,0")
                        .replace("5,10", "7,10")
                        .replace("7.5,5", "10.5,5")
                        .replace("2.5,5", "3.5,5");

                    // Bitmap large pour le A
                    let b = match letter {
                        'A' => "bitmap:5x7:010803040f080808".to_string(),
                        _ => "bitmap:5x7:010803040f080808".to_string(), // Par défaut utiliser le A large
                    };

                    (v, b, "Version large".to_string())
                }
                _ => {
                    // Variante stylisée avec courbes
                    let v = match letter {
                        'A' => "M(0,0)-C(5,15)-C(10,0)-L(8,5)-L(2,5)".to_string(),
                        _ => format!("M(0,0)-C(5,12)-C(10,0)-Z() letter:{}", letter),
                    };

                    // Bitmap stylisé pour le A
                    let b = match letter {
                        'A' => "bitmap:5x7:02050a041f111111".to_string(),
                        _ => "bitmap:5x7:02050a041f111111".to_string(), // Par défaut utiliser le A stylisé
                    };

                    (v, b, "Version stylisée avec courbes".to_string())
                }
            };

            let variant = GlyphVariant {
                bitmap,
                vector,
                notes: Some(format!("Variante #{}: {}", i, description)),
            };

            self.memory
                .glyph_knowledge
                .entry(letter)
                .or_insert(GlyphKnowledge {
                    letter,
                    variants: Vec::new(),
                    chosen_index: None,
                })
                .variants
                .push(variant);

            println!("Variante {}: {}", i, description);
        }

        println!("Généré {} variantes pour le glyphe '{}'", count, letter);
    }

    // Supprime toutes les variantes d'un glyphe
    pub fn clear_variants(&mut self, letter: char) {
        if let Some(entry) = self.memory.glyph_knowledge.get_mut(&letter) {
            entry.variants.clear();
            entry.chosen_index = None;
            println!(
                "Toutes les variantes du glyphe '{}' ont été supprimées",
                letter
            );
        }
        // Supprimer aussi de l'ancienne structure si présent
        self.memory.glyphs.remove(&letter.to_string());
    }

    // Compte le nombre de variantes pour un glyphe
    pub fn count_variants(&self, letter: char) -> usize {
        self.memory
            .glyph_knowledge
            .get(&letter)
            .map(|glyph| glyph.variants.len())
            .unwrap_or(0)
    }

    // Affiche les variantes disponibles pour un glyphe
    pub fn list_variants(&self, c: char) {
        if let Some(glyph) = self.memory.glyph_knowledge.get(&c) {
            println!("Glyph '{}': {} variant(s)", c, glyph.variants.len());
            for (i, variant) in glyph.variants.iter().enumerate() {
                println!("  [{}] {}", i, variant.bitmap);
            }
        } else {
            println!("Aucun glyphe pour '{}'", c);
        }
    }

    // Sélectionne une variante spécifique pour un glyphe
    pub fn choose_variant(&mut self, c: char, index: usize) {
        if let Some(glyph) = self.memory.glyph_knowledge.get_mut(&c) {
            if index < glyph.variants.len() {
                glyph.chosen_index = Some(index);

                // Mise à jour dans la mémoire `glyphs` classique
                let vector = &glyph.variants[index].vector;
                let bitmap = &glyph.variants[index].bitmap;
                let full = format!("vector:{}; {}", vector, bitmap);
                self.memory.glyphs.insert(c.to_string(), full);
            }
        }
    }

    // Génère une variante de glyphe et l'ajoute à la mémoire
    pub fn generate_glyph_variant(&mut self, c: char) {
        let vector = self.generate_vector_glyph(c);
        let bitmap = self.generate_bitmap_glyph(c);

        let variant = GlyphVariant {
            bitmap,
            vector,
            notes: None,
        };

        self.memory
            .glyph_knowledge
            .entry(c)
            .or_insert(GlyphKnowledge {
                letter: c,
                variants: Vec::new(),
                chosen_index: None,
            })
            .variants
            .push(variant);
    }

    fn generate_vector_glyph(&self, letter: char) -> String {
        match letter {
            'A' => "M(0,0)-L(5,10)-L(10,0)-L(7.5,5)-L(2.5,5)".into(),
            'B' => "M(0,0)-L(0,10)-L(7,10)-C(10,7)-C(10,3)-L(7,0)-Z()".into(),
            'C' => "M(10,2)-C(8,0)-C(2,0)-C(0,5)-C(2,10)-C(8,10)-C(10,8)".into(),
            // Plus de définitions pour D-Z et 0-9
            '0'..='9' => format!("M(0,0)-L(0,10)-L(10,10)-L(10,0)-Z() digit:{}", letter),
            'D'..='Z' => format!("M(0,0)-L(0,10)-L(10,10)-L(10,0)-Z() letter:{}", letter),
            _ => "M(0,0)-L(10,10)-M(0,10)-L(10,0)".into(), // X pour caractère inconnu
        }
    }

    fn generate_bitmap_glyph(&self, letter: char) -> String {
        // Utilisation d'un tableau d'octets pour représenter une grille 5x7
        // Chaque octet représente une ligne, avec les bits représentant les pixels
        let bitmap_data = match letter {
            'A' => [
                0b00100, 0b01010, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
            ],
            'B' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
            ],
            'C' => [
                0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
            ],
            'D' => [
                0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
            ],
            'E' => [
                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
            ],
            'F' => [
                0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
            ],
            'G' => [
                0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
            ],
            'H' => [
                0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
            ],
            'I' => [
                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
            ],
            'J' => [
                0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100,
            ],
            'K' => [
                0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
            ],
            'L' => [
                0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
            ],
            'M' => [
                0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
            ],
            'N' => [
                0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
            ],
            'O' => [
                0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
            ],
            'P' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
            ],
            'Q' => [
                0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
            ],
            'R' => [
                0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
            ],
            'S' => [
                0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
            ],
            'T' => [
                0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
            ],
            'U' => [
                0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
            ],
            'V' => [
                0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
            ],
            'W' => [
                0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001,
            ],
            'X' => [
                0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
            ],
            'Y' => [
                0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
            ],
            'Z' => [
                0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
            ],
            '0' => [
                0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
            ],
            '1' => [
                0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
            ],
            '2' => [
                0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
            ],
            '3' => [
                0b11111, 0b00010, 0b00100, 0b00010, 0b00001, 0b10001, 0b01110,
            ],
            '4' => [
                0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
            ],
            '5' => [
                0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110,
            ],
            '6' => [
                0b00110, 0b01000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
            ],
            '7' => [
                0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
            ],
            '8' => [
                0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
            ],
            '9' => [
                0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00010, 0b01100,
            ],
            _ => [
                0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b01010, 0b10001,
            ], // X pour caractère inconnu
        };

        // Conversion du bitmap en chaîne hexadécimale
        let hex_string = bitmap_data
            .iter()
            .map(|&byte| format!("{:02x}", byte))
            .collect::<Vec<String>>()
            .join("");

        format!("bitmap:5x7:{}", hex_string)
    }

    // Convertit un bitmap en chaîne lisible
    pub fn bitmap_to_string(&self, bitmap: &str) -> String {
        if !bitmap.starts_with("bitmap:") {
            return "Format invalide".to_string();
        }

        let parts: Vec<&str> = bitmap.split(':').collect();
        if parts.len() != 3 {
            return "Format invalide".to_string();
        }

        let dimensions: Vec<&str> = parts[1].split('x').collect();
        if dimensions.len() != 2 {
            return "Dimensions invalides".to_string();
        }

        let width = dimensions[0].parse::<usize>().unwrap_or(5);
        let hex_data = parts[2];

        let mut result = String::new();

        for i in 0..(hex_data.len() / 2) {
            if i * 2 + 2 <= hex_data.len() {
                if let Ok(byte) = u8::from_str_radix(&hex_data[i * 2..i * 2 + 2], 16) {
                    for bit_pos in 0..width {
                        let bit = (byte >> (width - 1 - bit_pos)) & 1;
                        result.push(if bit == 1 { 'X' } else { '.' });
                    }
                    result.push('\n');
                }
            }
        }

        result
    }

    // Génère tous les glyphes pour A-Z et 0-9
    pub fn generate_all_glyphs(&mut self) {
        // Générer A-Z
        for c in 'A'..='Z' {
            self.generate_glyph(c);
        }

        // Générer 0-9
        for c in '0'..='9' {
            self.generate_glyph(c);
        }
    }

    // Alias de count_glyph_variants pour compatibilité
    pub fn count_variants(&self, letter: char) -> usize {
        self.memory
            .glyph_knowledge
            .get(&letter)
            .map(|glyph| glyph.variants.len())
            .unwrap_or(0)
    }

    // Génère plusieurs variantes pour un même glyphe
    pub fn generate_multiple_variants(&mut self, letter: char, count: usize) {
        // D'abord, générer la variante standard
        self.generate_glyph_variant(letter);
        println!("Variante 0: Standard (originale)");

        // Ensuite, générer des variantes modifiées
        for i in 1..count {
            let (vector, bitmap, description) = match i % 4 {
                0 => (
                    // Variante originale
                    self.generate_vector_glyph(letter),
                    self.generate_bitmap_glyph(letter),
                    "Standard (originale)".to_string(),
                ),
                1 => {
                    // Variante étroite
                    let v = self
                        .generate_vector_glyph(letter)
                        .replace("10,0", "7,0")
                        .replace("5,10", "4,10")
                        .replace("7.5,5", "5.5,5")
                        .replace("2.5,5", "2,5");

                    // Bitmap étroit pour le A (exemple concret)
                    let b = match letter {
                        'A' => "bitmap:5x7:02040911110909".to_string(),
                        _ => "bitmap:5x7:02040911110909".to_string(), // Par défaut utiliser le A étroit
                    };

                    (v, b, "Version étroite".to_string())
                }
                2 => {
                    // Variante large
                    let v = self
                        .generate_vector_glyph(letter)
                        .replace("10,0", "14,0")
                        .replace("5,10", "7,10")
                        .replace("7.5,5", "10.5,5")
                        .replace("2.5,5", "3.5,5");

                    // Bitmap large pour le A
                    let b = match letter {
                        'A' => "bitmap:5x7:010803040f080808".to_string(),
                        _ => "bitmap:5x7:010803040f080808".to_string(), // Par défaut utiliser le A large
                    };

                    (v, b, "Version large".to_string())
                }
                _ => {
                    // Variante stylisée avec courbes
                    let v = match letter {
                        'A' => "M(0,0)-C(5,15)-C(10,0)-L(8,5)-L(2,5)".to_string(),
                        _ => format!("M(0,0)-C(5,12)-C(10,0)-Z() letter:{}", letter),
                    };

                    // Bitmap stylisé pour le A
                    let b = match letter {
                        'A' => "bitmap:5x7:02050a041f111111".to_string(),
                        _ => "bitmap:5x7:02050a041f111111".to_string(), // Par défaut utiliser le A stylisé
                    };

                    (v, b, "Version stylisée avec courbes".to_string())
                }
            };

            let variant = GlyphVariant {
                bitmap,
                vector,
                notes: Some(format!("Variante #{}: {}", i, description)),
            };

            self.memory
                .glyph_knowledge
                .entry(letter)
                .or_insert(GlyphKnowledge {
                    letter,
                    variants: Vec::new(),
                    chosen_index: None,
                })
                .variants
                .push(variant);

            println!("Variante {}: {}", i, description);
        }

        println!("Généré {} variantes pour le glyphe '{}'", count, letter);
    }

    // Supprime toutes les variantes d'un glyphe
    pub fn clear_glyph_variants(&mut self, letter: char) {
        if let Some(entry) = self.memory.glyph_knowledge.get_mut(&letter) {
            entry.variants.clear();
            entry.chosen_index = None;
            println!(
                "Toutes les variantes du glyphe '{}' ont été supprimées",
                letter
            );
        }
        // Supprimer aussi de l'ancienne structure si présent
        self.memory.glyphs.remove(&letter.to_string());
    }
}
