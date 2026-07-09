use std::fs;
use std::path::Path;

pub struct ProjectCreator {
    base_dir: String,
}

impl ProjectCreator {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub fn create(&self, id: usize) -> std::io::Result<()> {
        let project_dir = format!("{}/project_{:05}", self.base_dir, id);
        fs::create_dir_all(&project_dir)?;
        fs::create_dir_all(format!("{}/src", project_dir))?;
        Ok(())
    }
}
