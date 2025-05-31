mod backup;
mod types;
mod update;
mod utils;
mod content;
pub mod coordinator;  // rendre le module public

pub use types::*;
pub use utils::show_diff;
pub use content::generate_enum_content;
