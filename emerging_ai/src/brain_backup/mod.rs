mod memory; // Rendre memory.rs privé
mod memory_episode; // Rendre memory_episode.rs privé
mod memory_artifact; // Nouveau module pour les artefacts
pub mod persistents; // Expose le dossier persistents comme public

// Expose des éléments spécifiques pour simplifier les imports
pub use persistents::PersistentMemoryGraph; // Expose PersistentMemoryGraph directement