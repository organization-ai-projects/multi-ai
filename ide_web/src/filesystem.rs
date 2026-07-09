use std::fs;
use std::path::Path;
use crate::bridge::smart_bridge::revert_visual;

pub fn read_file(path: &str) -> Option<String> {
    let path = Path::new(path);
    fs::read_to_string(path).ok()
}

pub fn write_file(path: &str, content: &str) -> std::io::Result<()> {
    fs::write(path, content)
}

pub fn list_files(dir: &str) -> Option<Vec<String>> {
    let mut result = Vec::new();
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries {
        let entry = entry.ok()?;
        let path = entry.path();
        if path.is_file() {
            result.push(path.to_string_lossy().to_string());
        }
    }
    Some(result)
}

pub fn save_file_with_snapshot(path: &str, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)?;
    revert_visual(path); // Utilisation de smart_bridge pour gérer les snapshots
    Ok(())
}

pub fn is_supported_file(path: &str) -> bool {
    let extensions = ["rs", "js", "py", "html", "css"];
    if let Some(ext) = Path::new(path).extension() {
        extensions.contains(&ext.to_str().unwrap_or(""))
    } else {
        false
    }
}