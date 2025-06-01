mod core;
mod api;

pub use api::RustDbApi;

// Re-exports des types principaux pour les utilisateurs
pub use core::{Collection, Document, Query};