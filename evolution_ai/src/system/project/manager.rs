use super::templates::ProjectTemplates;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct ProjectManager {
    base_dir: PathBuf,
    templates: ProjectTemplates,
}

impl ProjectManager {
    pub fn new(base_dir: &str) -> Self {
        Self {
            base_dir: PathBuf::from(base_dir),
            templates: ProjectTemplates::new(),
        }
    }

    pub fn create_project(&self, id: usize) -> std::io::Result<PathBuf> {
        let ai_dir = self.base_dir.join(format!("ai_{:05}", id));
        self.templates.write_to_project(&ai_dir, id)?;
        Ok(ai_dir)
    }

    pub fn get_all_projects(&self) -> Vec<PathBuf> {
        fs::read_dir(&self.base_dir)
            .unwrap_or_else(|_| panic!("Dossier {} non trouvé", self.base_dir.display()))
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect()
    }
}
