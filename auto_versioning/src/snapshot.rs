use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::brain::VersioningBrain;  // Remplacer models::ChangeGraph par VersioningBrain

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Snapshot {
    version: String,
    timestamp: DateTime<Utc>,
    files: Vec<FileState>,
    #[serde(skip)] // Ignorer la sérialisation de brain_state
    brain_state: VersioningBrain,
}

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
struct FileState {
    path: PathBuf,
    content: Vec<u8>,
    hash: String,
}

pub struct SnapshotManager {
    snapshot_dir: PathBuf,
    current: Option<Snapshot>,
}

impl SnapshotManager {
    pub fn new(snapshot_dir: &Path) -> Self {
        std::fs::create_dir_all(snapshot_dir).unwrap_or_default();
        Self {
            snapshot_dir: snapshot_dir.to_owned(),
            current: None,
        }
    }

    pub fn take_snapshot(&mut self, project_dir: &Path) -> std::io::Result<()> {
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(project_dir) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let content = fs::read(entry.path())?;
                let hash = sha256::digest(&content);
                files.push(FileState {
                    path: entry.path().to_owned(),
                    content,
                    hash,
                });
            }
        }

        let snapshot = Snapshot {
            version: env!("CARGO_PKG_VERSION").to_string(),
            timestamp: Utc::now(),
            files,
            brain_state: VersioningBrain::load("brain_state"),  // Adapter pour utiliser VersioningBrain
        };

        let mut file = File::create(self.snapshot_dir.join(format!("snapshot_{}.bin", snapshot.version)))?;
        bincode_next::encode_into_std_write(&snapshot, &mut &mut file, bincode_next::config::standard())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        self.current = Some(snapshot);
        Ok(())
    }

    pub fn rollback(&self, version: &str) -> std::io::Result<()> {
        // ...implementation du rollback...
        Ok(())
    }
}
