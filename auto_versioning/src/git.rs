use std::path::Path;
use std::process::Command;

#[derive(Clone)]
pub struct GitManager {
    repo_path: String,
}

impl GitManager {
    pub fn new(path: &Path) -> Self {
        Self {
            repo_path: path.to_string_lossy().into_owned()
        }
    }

    pub fn commit(&self, message: &str) -> Result<(), std::io::Error> {
        Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(&self.repo_path)
            .status()?;
        Ok(())
    }

    pub fn tag(&self, version: &str) -> Result<(), std::io::Error> {
        Command::new("git")
            .arg("tag")
            .arg(version)
            .current_dir(&self.repo_path)
            .status()?;
        Ok(())
    }
}
