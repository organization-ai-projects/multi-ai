use super::types::*;
use std::fs;
use std::path::Path;

pub struct VersionManager {
    toml_path: std::path::PathBuf,
    backup_path: std::path::PathBuf,
}

impl VersionManager {
    pub fn new(toml_path: &Path) -> Self {
        Self {
            toml_path: toml_path.to_owned(),
            backup_path: toml_path.with_extension("toml.bak"),
        }
    }

    pub fn get_current_version(&self) -> Option<String> {
        let content = fs::read_to_string(&self.toml_path).ok()?;
        for line in content.lines() {
            if line.trim().starts_with("version") {
                return line.split('=')
                    .nth(1)
                    .map(|v| v.trim().trim_matches('"').to_string());
            }
        }
        None
    }

    pub fn patch_version(&self, version_file: &str) -> std::io::Result<()> {
        // Backup du Cargo.toml
        if let Ok(content) = std::fs::read_to_string(&self.toml_path) {
            let _ = std::fs::write(&self.backup_path, &content);
        }

        let target_version: VersionMeta = ron::from_str(
            &fs::read_to_string(version_file).expect("version.ron manquant")
        ).expect("RON invalide pour version");

        let current = self.get_current_version();
        if current == Some(target_version.version.clone()) {
            println!("✅ Version à jour ({}) — aucune modification.", target_version.version);
            return Ok(());
        }

        let content = fs::read_to_string(&self.toml_path).expect("Lecture Cargo.toml échouée");
        let patched = content
            .lines()
            .map(|l| {
                if l.trim().starts_with("version") {
                    format!("version = \"{}\"", target_version.version)
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(&self.toml_path, patched).expect("Écriture Cargo.toml échouée");
        println!("🔧 Version mise à jour dans Cargo.toml: {}", target_version.version);
        Ok(())
    }

    pub fn restore_backup(&self) -> std::io::Result<()> {
        if let Ok(content) = std::fs::read_to_string(&self.backup_path) {
            std::fs::write(&self.toml_path, content)?;
            std::fs::remove_file(&self.backup_path)?;
        }
        Ok(())
    }
}
