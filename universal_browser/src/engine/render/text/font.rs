//! Définitions des polices et informations sur les caractères

/// Informations de base sur la police
pub struct Font {
    pub char_width: u32,
    pub char_height: u32,
    pub spacing: u32,
}

impl Font {
    /// Crée une nouvelle police avec les dimensions spécifiées
    pub fn new(char_width: u32, char_height: u32, spacing: u32) -> Self {
        Self {
            char_width,
            char_height,
            spacing,
        }
    }

    /// Police par défaut pour le navigateur
    pub fn default() -> Self {
        Self {
            char_width: 8,
            char_height: 12,
            spacing: 1,
        }
    }

    /// Calcule la largeur totale d'un texte avec cette police
    pub fn text_width(&self, text: &str) -> u32 {
        text.len() as u32 * (self.char_width + self.spacing)
    }
}

/// Informations sur un caractère à dessiner
pub struct CharInfo {
    pub c: char,
    pub x: u32,
    pub y: u32,
    pub color: [u8; 4],
}
