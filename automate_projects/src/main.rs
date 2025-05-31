use clap::Parser;
use std::path::PathBuf;

mod cli_domains_manager;
mod config;
mod cli;

const PROJECT_PATH: &str = "automate_projects";

fn main() {
    let args = cli::Args::parse();
    let project_path = PathBuf::from(PROJECT_PATH);
    if let Err(e) = cli::run(args, &project_path) {
        eprintln!("❌ Erreur: {}", e);
        std::process::exit(1);
    }
}
