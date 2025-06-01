use crate::nosql_structural::collections::Document;
use crate::schema::projects::{ProjectType, ProjectDocument};
use crate::nosql_structural::references::DbRef;
use ron::de::from_str;
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ScannedProject {
    pub path: PathBuf,
    pub document: Document<ProjectDocument>,
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
                let doc = Document {
                    _id: Uuid::now_v7(),
                    created_at: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64,
                    data: project_doc,
                    references: None,
                };
                found.push(ScannedProject {
                    path: path.to_path_buf(),
                    document: doc,
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
