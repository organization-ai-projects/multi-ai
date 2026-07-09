//rust_universal/src/ai/manager_ai/launch_all.rs
use std::fs;
use std::process::Command;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Agent {
    id: String,
    active: bool,
}

pub fn launch_all(project_base: &str) -> anyhow::Result<()> {
    let config_path = PathBuf::from(project_base).join("ai/ai_config/active_agents.ron");

    // Vérifier si le fichier existe
    if !config_path.exists() {
        eprintln!("❌ Le fichier de configuration {:?} est introuvable.", config_path);
        eprintln!("💡 Veuillez d'abord créer une IA en utilisant la fonction `create_agent`.");
        return Ok(());
    }

    let content = fs::read_to_string(&config_path)?;
    let agents: Vec<Agent> = ron::from_str(&content)?;

    for agent in agents.iter().filter(|a| a.active) {
        let agent_dir = PathBuf::from(project_base).join("ai/agents").join(&agent.id);
        if agent_dir.exists() {
            println!("🚀 Launching IA: {}", agent.id);
            let status = Command::new("cargo")
                .arg("run")
                .current_dir(&agent_dir)
                .status()?;

            if !status.success() {
                eprintln!("❌ IA {} failed to start", agent.id);
            }
        } else {
            eprintln!("❌ Agent directory not found: {:?}", agent_dir);
        }
    }
    Ok(())
}