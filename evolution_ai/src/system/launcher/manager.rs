use super::process::ProcessManager;
use std::path::PathBuf;

#[derive(Clone)]
pub struct LauncherManager {
    batch_size: usize,
    process_manager: ProcessManager,
}

impl LauncherManager {
    pub fn new(batch_size: usize) -> Self {
        Self {
            batch_size,
            process_manager: ProcessManager::new(),
        }
    }

    pub fn run_all(&self, projects: Vec<PathBuf>) -> Vec<tokio::process::Child> {
        projects
            .chunks(self.batch_size)
            .flat_map(|batch| {
                batch
                    .iter()
                    .filter_map(|path| self.process_manager.spawn_process(path))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}
