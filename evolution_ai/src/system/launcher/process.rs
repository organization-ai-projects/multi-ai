use std::path::PathBuf;
use tokio::process::{Child, Command};

pub struct ProcessManager;

impl ProcessManager {
    pub fn new() -> Self {
        Self
    }

    pub fn spawn_process(&self, path: &PathBuf) -> Option<Child> {
        Command::new("cargo")
            .args(&["run", "--manifest-path"])
            .arg(path.join("Cargo.toml"))
            .spawn()
            .ok()
    }
}
