mod security;
mod agent_manager;

use security::SecurityManager;
use agent_manager::{AgentState, find_latest_agent};
use std::{io, thread::sleep, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub const PROJECT_BASE: &str = "experiment_ai";

fn main() -> io::Result<()> {
    println!("🚀 Démarrage du système...");
    let security = SecurityManager::new(PROJECT_BASE);
    security.ensure_dirs()?;

    println!("📂 Recherche du dernier agent...");
    let agent_path = find_latest_agent(&security)?;
    println!("✅ Agent trouvé: {}", agent_path.display());
    
    let mut current_state = AgentState::new(agent_path)?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Erreur configuration Ctrl-C");

    while running.load(Ordering::SeqCst) {
        println!("👀 Surveillance de l'agent {}", current_state.id);
        if let Some(new_path) = current_state.monitor(&security)? {
            // Arrêt propre de l'ancienne version
            current_state.process.kill()?;
            
            // Démarre la nouvelle version
            current_state = AgentState::new(new_path)?;
        }
        sleep(Duration::from_secs(1));
    }

    // Arrêt propre
    current_state.process.kill()?;
    Ok(())
}
