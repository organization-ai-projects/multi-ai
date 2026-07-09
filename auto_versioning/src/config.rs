use serde::Deserialize;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
pub struct AutoVersionConfig {
    pub ignore_paths: HashSet<String>,
    pub ignore_changes: HashSet<String>,
    pub git_enabled: bool,
    pub cargo_publish: bool,
    pub require_confirmation: bool,
    pub patterns: Vec<Pattern>,
}

impl AutoVersionConfig {
    pub fn load(project_root: &Path) -> Self {
        let config_path = project_root.join("auto_versioning.toml");
        if let Ok(content) = std::fs::read_to_string(config_path) {
            toml::from_str(&content).unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

impl Default for AutoVersionConfig {
    fn default() -> Self {
        Self {
            ignore_paths: ["target/", "tests/"].iter().map(|s| s.to_string()).collect(),
            ignore_changes: HashSet::new(),
            git_enabled: true,
            cargo_publish: false,
            require_confirmation: true,
            patterns: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub patterns: Vec<Pattern>,
    pub exclude: Vec<String>,
    pub post_publish: Vec<Command>,
    pub authors: Vec<Author>,
    pub cache_dir: Option<String>,
    pub publish_targets: Vec<PublishTarget>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Pattern {
    pub match_str: String,
    pub impact: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub cmd: String,
    pub args: Vec<String>,
    pub condition: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Author {
    pub github: String,
    pub cargo: String,
}

#[derive(Debug, Deserialize)]
pub enum PublishTarget {
    Crates,
    GitHub,
    Both,
}
