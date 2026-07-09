use std::path::Path;
use std::process::Command;
use crate::errors::Result;

#[derive(Debug, Clone)]
pub enum PublishTarget {
    Crates,
    Npm,
    VSCode,
    PyPI,
    GitHub,
    Both,
}

#[derive(Debug)]
pub enum PublishResult {
    Success,
    Failure(String),
}

pub struct Publisher {
    project_path: std::path::PathBuf,
    cargo_toml_backup: Option<String>,
}

impl Publisher {
    pub fn new(project_path: &Path) -> Self {
        Self {
            project_path: project_path.to_owned(),
            cargo_toml_backup: None,
        }
    }

    pub fn backup_cargo_toml(&mut self) -> std::io::Result<()> {
        let cargo_path = self.project_path.join("Cargo.toml");
        self.cargo_toml_backup = std::fs::read_to_string(&cargo_path).ok();
        Ok(())
    }

    pub fn rollback_if_needed(&self) -> std::io::Result<()> {
        if let Some(backup) = &self.cargo_toml_backup {
            let cargo_path = self.project_path.join("Cargo.toml");
            std::fs::write(cargo_path, backup)?;
        }
        Ok(())
    }

    pub fn publish(&self, target: &PublishTarget) -> Result<PublishResult> {
        match target {
            PublishTarget::Crates => {
                let status = Command::new("cargo")
                    .arg("publish")
                    .current_dir(&self.project_path)
                    .status()?;

                if !status.success() {
                    self.rollback_if_needed()?;
                    return Ok(PublishResult::Failure("La publication sur crates.io a échoué".into()));
                }
            }
            PublishTarget::VSCode => {
                let status = Command::new("vsce")
                    .arg("publish")
                    .current_dir(&self.project_path)
                    .status()?;

                if !status.success() {
                    self.rollback_if_needed()?;
                    return Ok(PublishResult::Failure("La publication VSCode a échoué".into()));
                }
            }
            PublishTarget::Npm => {
                let status = Command::new("npm")
                    .arg("publish")
                    .current_dir(&self.project_path)
                    .status()?;

                if !status.success() {
                    self.rollback_if_needed()?;
                    return Ok(PublishResult::Failure("La publication NPM a échoué".into()));
                }
            }
            PublishTarget::PyPI => {
                let status = Command::new("twine")
                    .args(&["upload", "dist/*"])
                    .current_dir(&self.project_path)
                    .status()?;

                if !status.success() {
                    self.rollback_if_needed()?;
                    return Ok(PublishResult::Failure("La publication PyPI a échoué".into()));
                }
            }
            PublishTarget::GitHub => {
                // Implémentation de la publication GitHub...
                return Ok(PublishResult::Success);
            }
            PublishTarget::Both => {
                self.publish(&PublishTarget::Crates)?;
                return self.publish(&PublishTarget::GitHub);
            }
        }
        Ok(PublishResult::Success)
    }
}

impl PublishTarget {
    pub async fn publish(&self, version: &str) -> Result<bool> {
        match self {
            Self::Crates | Self::Npm | Self::VSCode | Self::PyPI | Self::GitHub => {
                // Utiliser la même méthode pour tous les types
                self.publish_generic(version).await
            }
            Self::Both => {
                self.publish_generic(version).await?;
                Ok(true)
            }
        }
    }

    async fn publish_generic(&self, version: &str) -> Result<bool> {
        // Logique générique de publication...
        Ok(true)
    }
}
