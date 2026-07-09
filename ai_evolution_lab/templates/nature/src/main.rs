mod ecosystem;
mod environment;
mod execution;
mod life_cycle;
mod natural_selection;
mod survival_rules;
mod life_form;

use ecosystem::Ecosystem;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize)]
struct Config {
    id: String,
    kind: String,
    rules: Vec<String>,
    watch_path: String,
    results_path: String,
}

fn load_config() -> Config {
    ron::from_str(&fs::read_to_string("metadata.ron").unwrap()).unwrap()
}

fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Nature: Arrêt en cours...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Erreur configuration handler Ctrl+C");

    // Configuration
    let config = load_config();
    let env_path = PathBuf::from("environment");

    // Utilisation de l'écosystème comme orchestrateur
    let mut ecosystem = Ecosystem::new(env_path);
    println!("Nature [{}] démarre...", config.id);

    ecosystem.init();

    while running.load(Ordering::SeqCst) {
        ecosystem.process_cycle();
        thread::sleep(Duration::from_secs(1));
    }

    // Nettoyage propre avant de sortir
    ecosystem.clean_shutdown();
    println!("Nature: Arrêt terminé.");
}
