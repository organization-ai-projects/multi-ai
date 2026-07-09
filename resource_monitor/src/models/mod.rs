pub mod ai;
pub mod resources;
pub mod security;

// Re-exports pour faciliter l'accès aux modèles communs
pub use ai::{AiInstance, AiState};
pub use resources::ResourceThresholds;
pub use security::{SecurityLevel, SecurityAlert};
