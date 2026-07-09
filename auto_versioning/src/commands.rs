use std::process::Command;
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostCommand {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub condition: RunCondition,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub enum RunCondition {
    Always,
    OnSuccess,
    OnFailure,
}

pub struct CommandRunner {
    working_dir: std::path::PathBuf,
}

impl CommandRunner {
    pub fn new(dir: &Path) -> Self {
        Self { 
            working_dir: dir.to_owned() 
        }
    }

    pub fn run(&self, cmd: &PostCommand, success: bool) -> std::io::Result<()> {
        match (&cmd.condition, success) {
            (RunCondition::OnSuccess, false) => return Ok(()),
            (RunCondition::OnFailure, true) => return Ok(()),
            _ => {}
        }

        println!("🔄 Exécution de la commande post-publication: {}", cmd.name);
        let status = Command::new(&cmd.command)
            .args(&cmd.args)
            .current_dir(&self.working_dir)
            .status()?;

        if !status.success() {
            eprintln!("❌ La commande '{}' a échoué", cmd.name);
        }
        Ok(())
    }
}
