#[derive(Debug, Clone)]
pub struct DetectedModule {
    pub name: String,
    pub module_type: ModuleType,
}

#[derive(Debug, Clone)]
pub enum ModuleType {
    Domain(String),
    System,
}

#[derive(Default)]
pub struct ModificationTracker {
    pub mod_rs_changes: bool,
    pub trait_commands_changes: bool,
    pub total_changes: usize,
}
