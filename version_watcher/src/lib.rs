pub mod cli;
pub mod graph;
pub mod store;
pub mod version;
pub mod watcher;

pub use cli::run; // point d’entrée unique réexporté
