// src/main.rs

// mod models;  // Supprimer ce mod si vous utilisez models/mod.rs
mod analyzer;
mod watcher;
mod versioning;
mod changelog;
mod config;
mod git;
mod cache;
mod cli;
mod backup;
mod webhook;
mod snapshot;
mod workspace;
mod commands;
mod errors;
mod api;
mod graph;
mod publishing;
mod init;
mod brain;  // Ajouter le module brain
mod setup;
mod app;
mod monitoring; // Ajouter cette ligne

use crate::cli::CliArgs;
use crate::errors::Result;
use crate::app::Application;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialiser le logging
    tracing_subscriber::fmt::init();

    // Parser les arguments
    let args = CliArgs::parse();

    // Initialiser l'application
    let components = setup::initialize(args).await?;
    let app = Application::new(components);

    // Démarrer l'application
    app.run().await
}
