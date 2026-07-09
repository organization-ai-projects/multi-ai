// Module de rendu divisé en plusieurs fichiers spécialisés
pub mod page;
pub mod primitives; // Formes de base (rectangles, lignes)
pub mod text; // Rendu de texte réorganisé
pub mod ui; // Composants d'interface utilisateur

// Re-exporter les éléments importants pour simplifier les imports
pub use page::render_page;
pub use text::draw_text; // Simplifier le chemin d'export
pub use ui::UiState;
