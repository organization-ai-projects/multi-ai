use super::launcher::LauncherManager;
use super::project::ProjectManager;
use std::path::PathBuf;

pub struct SystemManager {
    project_manager: ProjectManager,
    launcher: LauncherManager,
}

impl SystemManager {
    pub fn new(base_dir: &str) -> Self {
        Self {
            project_manager: ProjectManager::new(base_dir),
            launcher: LauncherManager::new(10),
        }
    }

    pub fn create_project(&self, id: usize) -> std::io::Result<PathBuf> {
        self.project_manager.create_project(id)
    }

    pub fn run_processes(&self) -> Vec<tokio::process::Child> {
        self.launcher
            .run_all(self.project_manager.get_all_projects())
    }
}
