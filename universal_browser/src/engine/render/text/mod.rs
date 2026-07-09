//! Module pour le rendu de texte
//! Ce module gère tout ce qui concerne l'affichage de texte dans le navigateur

pub mod draw; // Rendre le module public
pub mod font; // Rendre le module public
mod glyphs;

// Réexporter les fonctions et structures publiques
pub use draw::draw_text;
pub use font::{CharInfo, Font};
