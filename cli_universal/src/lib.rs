pub mod api;
pub mod commands;

// Tu exposes ici ce que tu veux rendre public pour d’autres crates Rust :
pub use api::*;
pub use commands::ai::{AiCommands, dispatch as ai_dispatch};
pub use commands::agents::{AgentsCommands, dispatch as agents_dispatch};
