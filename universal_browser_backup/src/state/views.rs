//! # Gestion centralisée des vues
//!
//! Ce fichier définit les vues de l'application et leurs métadonnées.

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppView {
    Welcome,
    IAExplorer,
    GraphMemory,
    FileViewer,
}

impl AppView {
    /// Retourne l'ID de l'option de sidebar associée à la vue
    pub fn sidebar_option_id(&self) -> &'static str {
        match self {
            AppView::Welcome => "option_welcome",
            AppView::IAExplorer => "option_ia",
            AppView::GraphMemory => "option_graph",
            AppView::FileViewer => "option_files",
        }
    }

    /// Retourne l'URI associée à la vue
    pub fn uri(&self) -> &'static str {
        match self {
            AppView::Welcome => "welcome://home",
            AppView::IAExplorer => "ia://explorer",
            AppView::GraphMemory => "graph://memory",
            AppView::FileViewer => "file://explorer",
        }
    }
}
