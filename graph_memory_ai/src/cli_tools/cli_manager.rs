use std::process::{Command, Output};
use super::cli_commands::{CommandInfo, execute_command, get_commands};

pub struct CliTool {
    pub name: String,
    pub description: String,
    pub path: String,
}

pub fn get_available_tools() -> Vec<CliTool> {
    vec![
        CliTool {
            name: "automate_projects".into(),
            description: "Analyse et met à jour les projets".into(),
            path: "cargo run -p automate_projects --".into(),
        }
    ]
}

pub fn execute_tool(tool: &str, args: &[&str]) -> Result<Output, String> {
    match tool {
        "automate_projects" => {
            Command::new("cargo")
                .args(["run", "-p", "automate_projects", "--"])
                .args(args)
                .output()
                .map_err(|e| format!("Erreur exécution {}: {}", tool, e))
        }
        _ => Err(format!("Outil inconnu: {}", tool))
    }
}

pub struct AiCliContext {
    last_command: Option<String>,
    available_commands: Vec<CommandInfo>,
}

impl AiCliContext {
    pub fn new() -> Self {
        Self {
            last_command: None,
            available_commands: get_commands(),
        }
    }

    pub fn execute(&mut self, command: &str, args: &[&str]) -> Result<String, String> {
        let output = execute_command(command, args)?;
        self.last_command = Some(command.to_string());
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn get_available_commands(&self) -> &[CommandInfo] {
        &self.available_commands
    }
}
