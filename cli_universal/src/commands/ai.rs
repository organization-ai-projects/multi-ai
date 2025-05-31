use clap::{Subcommand, Args};

#[derive(Subcommand)]
pub enum AiCommands {
    List,
    Create { name: String },
    Status { name: Option<String> },
}

pub fn dispatch(cmd: &AiCommands) {
    match cmd {
        AiCommands::List => {
            println!("Liste des IA...");
            // Ici tu peux appeler ta fonction API Rust ou logique métier
        },
        AiCommands::Create { name } => {
            println!("Création de l’IA : {}", name);
            // Appel à ta fonction d’API interne ou création d’agent
        },
        AiCommands::Status { name } => {
            if let Some(name) = name {
                println!("Statut de l’IA : {}", name);
                // Appel API interne
            } else {
                println!("Statut de toutes les IA :");
                // Appel API interne
            }
        },
    }
}
