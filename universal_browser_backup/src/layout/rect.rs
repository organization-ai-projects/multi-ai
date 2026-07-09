//! # Définition des rectangles pour le layout
//! 
//! Ce fichier est responsable de:
//! - Définir la structure de base des rectangles (x, y, width, height)
//! - Fournir des méthodes utilitaires pour les rectangles (contient un point, etc.)
//! 
//! Ce fichier NE DOIT PAS contenir:
//! - Des informations de rendu ou de couleur
//! - Des logiques de layout complexes
//! 
//! Il fournit uniquement la base géométrique pour le système de layout.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}
