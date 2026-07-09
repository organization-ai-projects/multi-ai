pub mod analyzer;
pub mod classifier;
pub mod changelog;
pub mod validator;
pub mod graph_memory;
pub mod agent;
pub mod actions;
pub mod cli;

pub use analyzer::analyze_snapshot;
pub use classifier::classify_impact;
pub use changelog::generate_changelog;
pub use validator::validate_bump;
pub use graph_memory::{AiGraph, AiEvent};
pub use agent::{decide_and_act, generate_session_report, run_agent};
pub use actions::{bump_version, revert_to};
pub use cli::parse_and_execute;
