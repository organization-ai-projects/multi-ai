//! # Définition et gestion des couleurs
//! 
//! Ce fichier est responsable de:
//! - Définir la structure des couleurs (RGBA)
//! - Fournir des couleurs prédéfinies
//! - Offrir des méthodes utilitaires pour manipuler les couleurs
//! 
//! Ce fichier NE DOIT PAS contenir:
//! - Des fonctions de dessin
//! - Des logiques de layout
//! - Des codes de rendu de pixels
//! 
//! Il se concentre uniquement sur la définition et manipulation des couleurs.

// Définition des couleurs et leurs fonctions utilitaires

// Couleur RGBA
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    // Couleurs prédéfinies
    pub fn red() -> Self {
        Self::new(255, 0, 0, 255)
    }

    pub fn green() -> Self {
        Self::new(0, 255, 0, 255)
    }

    pub fn blue() -> Self {
        Self::new(0, 0, 255, 255)
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0, 255)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255, 255)
    }

    pub fn transparent() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn gray(intensity: u8) -> Self {
        Self::new(intensity, intensity, intensity, 255)
    }
}
