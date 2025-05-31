use std::process::{Command, Output};

// Structure pour stocker les informations d'une commande
#[derive(Clone)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
}

// Liste des commandes disponibles
pub fn get_commands() -> Vec<CommandInfo> {
    vec![
        CommandInfo {
            name: "automate_projects".into(),
            description: "Analyse et met à jour les projets".into(),
        }
    ]
}

// Exécution directe des commandes
pub fn execute_command(command: &str, args: &[&str]) -> Result<Output, String> {
    match command {
        "automate_projects" => {
            Command::new("cargo")
                .args(["run", "-p", "automate_projects", "--"])
                .args(args)
                .output()
                .map_err(|e| format!("Erreur: {}", e))
        }
        _ => Err(format!("Commande inconnue: {}", command)),
    }
}
