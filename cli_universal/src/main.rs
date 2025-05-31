mod commands;
mod api;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Universal CLI")]
#[command(about = "Orchestrateur Rust multi-agents/IA/outils", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Ai(commands::ai::AiCommands),
    #[command(subcommand)]
    Agents(commands::agents::AgentsCommands),
    // ... Ajoute d'autres domaines ici
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Ai(cmd)) => commands::ai::dispatch(cmd),
        Some(Commands::Agents(cmd)) => commands::agents::dispatch(cmd),
        None => println!("Aucune commande spécifiée. Utilise --help pour la liste."),
    }
}
