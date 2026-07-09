use std::path::{Path, PathBuf};
use chrono::Local;
use std::fs;
use std::process::Command;

pub struct ChangelogManager {
    changelog_path: PathBuf,
}

impl ChangelogManager {
    pub fn new(project_root: &Path) -> Self {
        Self {
            changelog_path: project_root.join("CHANGELOG.md")
        }
    }

    pub fn add_entry(&self, version: &str, changes: &[(String, String)]) -> std::io::Result<()> {
        let date = Local::now().format("%Y-%m-%d");
        let mut content = if self.changelog_path.exists() {
            fs::read_to_string(&self.changelog_path)?
        } else {
            "# Changelog\n\n".to_string()
        };

        let mut entry = format!("\n## [{}] - {}\n\n", version, date);
        
        // Grouper par type d'impact
        let mut major = Vec::new();
        let mut minor = Vec::new();
        let mut patch = Vec::new();

        for (path, impact) in changes {
            let change = path.split('/').last().unwrap_or(path);
            match impact.as_str() {
                "major" => major.push(change),
                "minor" => minor.push(change),
                "patch" => patch.push(change),
                _ => {}
            }
        }

        if !major.is_empty() {
            entry.push_str("### Breaking Changes\n\n");
            for change in major {
                entry.push_str(&format!("- {} changements majeurs\n", change));
            }
        }

        if !minor.is_empty() {
            entry.push_str("\n### Features\n\n");
            for change in minor {
                entry.push_str(&format!("- {} nouvelles fonctionnalités\n", change));
            }
        }

        if !patch.is_empty() {
            entry.push_str("\n### Bug Fixes\n\n");
            for change in patch {
                entry.push_str(&format!("- {} corrections\n", change));
            }
        }

        content.insert_str(content.find("## ").unwrap_or(content.len()), &entry);
        fs::write(&self.changelog_path, content)
    }

    pub fn generate_from_git(&self, version: &str) -> std::io::Result<()> {
        let output = Command::new("git")
            .args(&["log", "--pretty=format:%s", "HEAD...HEAD~10"])
            .current_dir(&self.changelog_path.parent().unwrap())
            .output()?;

        let commits = String::from_utf8_lossy(&output.stdout);
        let changes = commits.lines()
            .map(|msg| {
                let impact = if msg.contains("break") {
                    "major"
                } else if msg.contains("feat") {
                    "minor"
                } else {
                    "patch"
                };
                (msg.to_string(), impact.to_string())
            })
            .collect::<Vec<_>>();

        self.add_entry(version, &changes)
    }

    pub fn update_badge(&self, version: &str) -> std::io::Result<()> {
        let badge = format!(
            "![Version](https://img.shields.io/badge/version-{}-blue.svg)",
            version
        );
        
        let readme_path = self.changelog_path.parent().unwrap().join("README.md");
        if let Ok(content) = fs::read_to_string(&readme_path) {
            let updated = regex::Regex::new(r"!\[Version\].*\n")
                .unwrap()
                .replace(&content, &format!("{}\n", badge));
            fs::write(readme_path, updated.as_ref())?;
        }
        Ok(())
    }
}
