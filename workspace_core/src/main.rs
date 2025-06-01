mod commands;
mod scanner;
mod schema;

use clap::{Parser, Subcommand};
use schema::{Collection, Document};
use ron::de::from_str;
use std::fs;

#[derive(Parser)]
#[command(name = "workspace")]
#[command(about = "Gestionnaire de workspace Multi-AI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        name: String,
    },
    Scan,
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => {
            if let Err(e) = commands::init_workspace(name) {
                eprintln!("Erreur initialisation : {}", e);
            }
        }
        Commands::Scan => {
            if let Ok(content) = fs::read_to_string("workspace.ron") {
                if let Ok(collection) = from_str::<Collection>(&content) {
                    if let Err(e) = commands::scan_workspace(&collection) {
                        eprintln!("Erreur scan : {}", e);
                    }
                }
            }
        }
        Commands::List => {
            if let Ok(content) = fs::read_to_string("workspace.ron") {
                if let Ok(collection) = from_str::<Collection>(&content) {
                    println!("\nProjets dans la collection {} :", collection.name);
                    for doc in collection.documents {
                        println!("• {} ({:?})", doc.name, doc._id);
                    }
                }
            }
        }
    }
}
