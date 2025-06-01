mod commands;
mod scanner;
mod schema;
mod nosql_structural;

use std::collections::HashMap;
use clap::{Parser, Subcommand};
use crate::nosql_structural::{collections::Collection, storage::StorageManager};
use crate::schema::ProjectDocument;
use ron::de::from_str;
use std::fs;
use std::path::Path;
use uuid::Uuid;

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
            let storage = StorageManager::new(Path::new(".").to_path_buf());
            let collection = storage
                .load::<schema::ProjectDocument>("workspace", true)
                .unwrap_or_else(|_| ProjectsCollection::new_workspace());

            if let Err(e) = commands::scan_workspace(&collection) {
                eprintln!("Erreur scan : {}", e);
            }
        }
        Commands::List => {
            if let Ok(content) = fs::read_to_string("workspace.ron") {
                if let Ok(collection) = from_str::<Collection<ProjectDocument>>(&content) {
                    println!("\nProjets dans la collection {} :", collection.name);
                    for doc in collection.documents {
                        println!("• {} ({:?})", doc.data.name, doc._id);
                    }
                }
            }
        }
    }
}
