mod types;
mod observer;
mod analysis;

pub use types::{Observation, ObservationType};
pub use observer::Observer;

// Re-export seulement ce qui est nécessaire
pub(crate) use analysis::SpecimenAnalysis;
