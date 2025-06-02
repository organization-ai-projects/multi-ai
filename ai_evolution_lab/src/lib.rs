mod models;
mod metadata;
mod setup;
mod runner;

// Expose uniquement ce qui est nécessaire à l'extérieur
pub use models::{IaList, IaMetadata, GlobalMetadata};
pub use metadata::{load_or_init_global_metadata, save_global_metadata};
pub use setup::{ensure_directories, generate_initial_metadata};
pub use runner::launch_ias;
