use tracing::Level; // Remplacer log par tracing
use std::path::PathBuf;
use crate::publishing;

pub struct CliArgs {
    pub target_project: PathBuf,
    pub force_publish: bool,
    pub dry_run: bool,
    pub manual_bump: Option<String>,
    pub publish_target: Option<PublishTarget>,
    pub skip_git: bool,
    pub test_rules: bool,
    pub discord_webhook: Option<String>,
    pub github_webhook: Option<String>,
    pub auto_commit: bool,
    pub log_level: Level,
}

#[derive(Clone)]
pub enum PublishTarget {
    Crates,
    GitHub,
    Both,
}

impl From<PublishTarget> for publishing::PublishTarget {
    fn from(target: PublishTarget) -> Self {
        match target {
            PublishTarget::Crates => publishing::PublishTarget::Crates,
            PublishTarget::GitHub => publishing::PublishTarget::GitHub,
            PublishTarget::Both => publishing::PublishTarget::Both,
        }
    }
}

impl CliArgs {
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        
        Self {
            force_publish: args.iter().any(|arg| arg == "--force-publish"),
            dry_run: args.iter().any(|arg| arg == "--dry-run"),
            manual_bump: args.iter()
                .find(|arg| arg.starts_with("--bump="))
                .map(|arg| arg.split('=').nth(1).unwrap().to_string()),
            publish_target: args.iter()
                .find(|arg| arg.starts_with("--publish="))
                .map(|arg| arg.split('=').nth(1).unwrap())
                .map(|t| match t {
                    "crates" => PublishTarget::Crates,
                    "gh" => PublishTarget::GitHub,
                    _ => PublishTarget::Both,
                }),
            skip_git: args.iter().any(|arg| arg == "--no-tag"),
            test_rules: args.iter().any(|arg| arg == "--test-rules"),
            target_project: args.iter()
                .find(|arg| !arg.starts_with("--"))
                .map(PathBuf::from)
                .unwrap_or_else(|| Self::show_usage()),
            discord_webhook: None, // Default value, can be changed by the user
            github_webhook: None,  // Default value, can be changed by the user
            auto_commit: false,    // Default value, can be changed by the user
            log_level: Level::INFO, // Default log level
        }
    }

    fn show_usage() -> PathBuf {
        println!("📦 Usage: auto_versioning <project_path> [options]");
        println!("ℹ️  Exemple: auto_versioning ../my_project");
        println!("💡 Options:");
        println!("   --force-publish    Force la publication même en self-watch");
        println!("   --dry-run         N'effectue pas de modifications");
        println!("   --bump=<version>  Définit une version manuelle");
        println!("   --publish=<target> [crates|gh|both]");
        println!("   --no-tag          Skip la création de tag Git");
        println!("   --test-rules      Teste les règles de pattern matching");
        PathBuf::from(".")
    }
}
