mod ai;
mod human;

use crate::ai::orchestrator::Orchestrator;
use crate::human::human_orchestrator::HumanOrchestrator;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    // Flag pour gérer l'arrêt propre
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // Gestionnaire CTRL+C
    ctrlc::set_handler(move || {
        println!("\nReçu CTRL+C, arrêt propre en cours...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Erreur lors de la configuration du gestionnaire CTRL+C");

    // Création des orchestrateurs
    let mut ai_orchestrator = Orchestrator::new();
    let mut human_orchestrator = HumanOrchestrator::new();

    // Boucle principale
    while running.load(Ordering::SeqCst) {
        // 1. Cycle de l'IA (asynchrone)
        let ai_cycle = tokio::spawn(async move {
            if let Err(e) = ai_orchestrator.run_autonomous_cycle().await {
                eprintln!("Erreur dans le cycle IA: {}", e);
            }
        });

        // 2. Monitoring humain (asynchrone)
        let human_cycle = tokio::spawn(async move {
            if let Err(e) = human_orchestrator.monitor_cycle().await {
                eprintln!("Erreur dans le monitoring: {}", e);
            }
        });

        // Attente des deux cycles
        tokio::try_join!(ai_cycle, human_cycle).unwrap();

        // Pause entre les cycles
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    // Nettoyage final
    println!("Arrêt des systèmes...");
    ai_orchestrator.cleanup().unwrap();
    println!("Arrêt terminé.");
}
