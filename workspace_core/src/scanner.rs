use rustdb::api::Database;
use crate::schema::ProjectDocument;
use ron::de::from_str;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ScannedProject {
    pub path: PathBuf,
    pub data: ProjectDocument,
}

fn scan_dir_recursive(path: &Path, excluded: &[String], found: &mut Vec<ScannedProject>) {
    if !path.is_dir() {
        return;
    }

    if let Some(dir_name) = path.file_name().and_then(|name| name.to_str()) {
        if excluded.contains(&dir_name.to_string()) {
            return;
        }
    }

    let meta_path = path.join("metadata.ron");
    if meta_path.exists() {
        if let Ok(content) = fs::read_to_string(&meta_path) {
            if let Ok(project_doc) = from_str::<ProjectDocument>(&content) {
                found.push(ScannedProject {
                    path: path.to_path_buf(),
                    data: project_doc,
                });
            }
        }
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                scan_dir_recursive(&entry.path(), excluded, found);
            }
        }
    }
}

pub fn scan_projects(path: &Path, excluded: &[String]) -> Vec<ScannedProject> {
    let mut found = Vec::new();
    scan_dir_recursive(path, excluded, &mut found);
    found
}
