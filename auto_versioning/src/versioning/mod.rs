mod change;
pub mod types;
pub mod manager;

pub use change::ChangeNode;
pub use types::{VersionMeta, ChangeSet};
pub use manager::VersionManager;
