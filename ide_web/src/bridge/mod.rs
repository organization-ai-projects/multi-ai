mod ai_bridge;
mod version_bridge;
mod errors; // Maintien du module `errors`
pub mod smart_bridge;

// Re-exports pour faciliter l'utilisation
pub use smart_bridge::{revert_visual, smart_version_bump, suggest_version_strategy};
pub use errors::BridgeError; // Réexport de `BridgeError`
