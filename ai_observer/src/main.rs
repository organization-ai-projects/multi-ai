//ai_observer/src/main.rs
mod api;
mod entities;
mod interfaces;

use crate::api::system_api::SystemAPI;
use crate::interfaces::cli::cli_interface::CLIInterface;

const STORAGE_PATH: &str = "memory/storage/knowledge_graph";

fn main() {
    println!("=== Système IA Observer ===");

    // Initialiser l'API système
    let api = SystemAPI::new(STORAGE_PATH);

    // Créer une interface CLI et lui passer l'API
    let mut cli = CLIInterface::new(api);

    // Exécuter l'interface
    cli.run();
}
