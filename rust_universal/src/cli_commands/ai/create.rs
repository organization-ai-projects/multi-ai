use crate::ai::manager_ai::create_agent::create_agent;
use clap::Command;

pub fn get_create_command() -> Command {
    Command::new("create")
        .about("Crée une nouvelle IA en utilisant un modèle prédéfini")
        .action(|| {
            if let Err(e) = create_agent() {
                eprintln!("❌ Erreur : {}", e);
            }
        })
}
