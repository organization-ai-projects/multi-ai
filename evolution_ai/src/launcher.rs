use std::path::Path;
use tokio::process::Command as TokioCommand;

#[derive(Clone)]
pub struct Launcher {
    batch_size: usize,
}

impl Launcher {
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    pub fn run_all_async(&self, ai_paths: Vec<std::path::PathBuf>) -> Vec<tokio::process::Child> {
        ai_paths
            .chunks(self.batch_size)
            .flat_map(|batch| {
                batch
                    .iter()
                    .filter_map(|ai_path| self.spawn_ai_process(ai_path))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn spawn_ai_process(&self, ai_path: &Path) -> Option<tokio::process::Child> {
        TokioCommand::new("cargo")
            .args(&["run", "--manifest-path"])
            .arg(ai_path.join("Cargo.toml"))
            .spawn()
            .ok()
    }
}
