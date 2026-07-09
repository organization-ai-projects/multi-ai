mod evolution;
mod orchestrator;
mod system;

use chrono::Utc;
use orchestrator::Orchestrator;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicBool, Ordering},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ajout du flag pour le Ctrl+C
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("\nArrêt demandé, attente de la fin de la génération...");
        r.store(false, Ordering::SeqCst);
    })?;

    // Configuration et initialisation de l'orchestrateur
    let config = orchestrator::RunConfig {
        run_id: format!("run_{}", Utc::now().format("%Y%m%d_%H%M%S")),
        population_size: 100,
        archive_generations: true,
        checkpoint_interval: 10,
        keep_archives: 50,
    };

    let mut orchestrator = Orchestrator::new(config);

    // Démarrer l'orchestrateur (cela bloquera jusqu'à l'arrêt)
    orchestrator.start(running).await;

    // Retourner Ok à la fin
    Ok(())
}
