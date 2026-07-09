use std::collections::HashMap;
use std::path::{Path, PathBuf};
use toml::Value;
use tracing::debug;  // Changer log::debug en tracing::debug

pub struct WorkspaceManager {
    root: PathBuf,
    members: Vec<PathBuf>,
    caches: HashMap<PathBuf, ProjectState>,
}

#[derive(Debug)]
struct ProjectState {
    last_check: std::time::SystemTime,
    cargo_toml: String,
    version: String,
}

impl WorkspaceManager {
    pub fn new(root: &Path) -> std::io::Result<Self> {
        let cargo_toml = root.join("Cargo.toml");
        let content = std::fs::read_to_string(cargo_toml)?;
        let value: toml::Value = toml::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let members: Vec<PathBuf> = value
            .get("workspace")
            .and_then(|w| w.get("members"))
            .and_then(|m| m.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| root.join(s))
                    .collect()
            })
            .unwrap_or_default();

        debug!("Workspace members trouvés: {:?}", members);
        
        // Initialisation des états des projets
        let mut caches = HashMap::new();
        for member in members.iter() {
            if let Ok(state) = Self::init_project_state(member.as_path()) {
                caches.insert(member.clone(), state);
            }
        }

        Ok(Self { 
            root: root.to_owned(), 
            members,
            caches 
        })
    }

    fn init_project_state(project_path: &Path) -> std::io::Result<ProjectState> {
        let cargo_path = project_path.join("Cargo.toml");
        let content = std::fs::read_to_string(&cargo_path)?;
        let version = Self::extract_version(&content)
            .unwrap_or_else(|| "0.1.0".to_string());

        Ok(ProjectState {
            last_check: std::time::SystemTime::now(),
            cargo_toml: content,
            version,
        })
    }

    fn extract_version(content: &str) -> Option<String> {
        let value: Value = toml::from_str(content).ok()?;
        value.get("package")?
            .get("version")?
            .as_str()
            .map(String::from)
    }

    pub fn watch_all(&self) -> Vec<PathBuf> {
        self.members.clone()
    }

    pub fn get_project_state(&self, path: &Path) -> Option<&ProjectState> {
        self.caches.get(path)
    }

    pub fn needs_update(&self, path: &Path) -> bool {
        if let Some(state) = self.caches.get(path) {
            if let Ok(metadata) = std::fs::metadata(path.join("Cargo.toml")) {
                if let Ok(modified) = metadata.modified() {
                    return modified > state.last_check;
                }
            }
        }
        true
    }

    pub fn backup_cargo_toml(&self, path: &Path) -> std::io::Result<()> {
        let cargo_path = path.join("Cargo.toml");
        let backup_path = path.join("Cargo.toml.bak");
        std::fs::copy(cargo_path, backup_path)?;
        Ok(())
    }

    pub fn restore_cargo_toml(&self, path: &Path) -> std::io::Result<()> {
        let cargo_path = path.join("Cargo.toml");
        let backup_path = path.join("Cargo.toml.bak");
        if backup_path.exists() {
            std::fs::rename(backup_path, cargo_path)?;
        }
        Ok(())
    }
}
