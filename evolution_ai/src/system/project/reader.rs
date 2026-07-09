use std::fs;
use std::path::Path;

pub struct ProjectReader {
    base_dir: String,
}

impl ProjectReader {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub fn read_output(&self, id: usize) -> std::io::Result<String> {
        fs::read_to_string(format!("{}/project_{:05}/output.txt", self.base_dir, id))
    }

    pub fn list_projects(&self) -> std::io::Result<Vec<std::path::PathBuf>> {
        Ok(fs::read_dir(&self.base_dir)?
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .collect())
    }
}
