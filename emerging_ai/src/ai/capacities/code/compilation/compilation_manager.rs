use std::path::PathBuf;
use std::process::Command;

pub struct CompilationManager {
    sandbox_dir: PathBuf,
}

impl CompilationManager {
    pub fn new() -> Self {
        Self {
            sandbox_dir: PathBuf::from("./sandbox"),
        }
    }

    pub fn compile(&self, code: &str, project_dir: &PathBuf) -> bool {
        Command::new("cargo")
            .args(["build"])
            .current_dir(project_dir)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
