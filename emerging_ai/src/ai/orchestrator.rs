//utilise memory/brain_api.rs et capacities/capacities_api.rs
//ce fichier est l'orchestrateur principal du domaine principal ai/

use crate::brain::brain_api::BrainAPI;
use crate::capacities::capacities_api::CapacitiesAPI;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::time::interval;
use uuid::Uuid;

pub struct Orchestrator {
    brain: BrainAPI,
    debug_mode: bool,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            brain: BrainAPI::new(),
            debug_mode: false,
        }
    }

    // Renommer l'ancienne méthode run_autonomous_loop en run_autonomous_cycle
    pub fn run_autonomous_cycle(&mut self) -> std::io::Result<()> {
        let cycle_start = Instant::now();

        // Vérification santé
        // Exécution du cycle
        let artifacts_before = self.count_artifacts();
        let result = self.execute_autonomous_cycle();
        let artifacts_produced = self.count_artifacts() - artifacts_before;

        // Collecte métriques
        // Debug mode
        if self.debug_mode {
            println!("Cycle completed. Press Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }

        // Génération status.json
        self.update_status_file()?;

        result
    }

    pub async fn run_async_cycle(&mut self) -> std::io::Result<()> {
        let learning_cycle = tokio::spawn(self.execute_autonomous_cycle());
        let cycle_result = learning_cycle.await?;

        // Utiliser BrainAPI pour vérifier l'état critique
        if self.brain.is_in_emergency() {
            self.brain
                .handle_critical_error("Critical state detected during cycle")?;
        }

        Ok(())
    }

    async fn execute_autonomous_cycle(&mut self) -> std::io::Result<()> {
        let capacities = CapacitiesAPI::new();

        // 1. L'IA utilise une capacité
        let result = capacities.code_manager.mutate("...")?;

        // 2. Le cerveau réagit automatiquement aux résultats via ses observations
        // Sans que les capacités aient à le savoir
        self.brain.process_signal(result);

        Ok(())
    }

    fn handle_error(&mut self, error: std::io::Error) -> std::io::Result<()> {
        // Log l'erreur
        let error_uuid = Uuid::now_v7(); // Modification ici
        self.brain.save_log_legacy(format!(
            "Error at {}: {}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            error
        ))?;

        // Tente une récupération depuis le dernier point stable
        self.brain.restore_from_journal()?;

        Ok(())
    }

    // Ajouter méthode de nettoyage
    pub fn cleanup(&mut self) -> std::io::Result<()> {
        // Sauvegarde finale
        self.brain.commit_transaction()?;

        // Flush des caches
        self.brain.flush_all()?;

        Ok(())
    }

    async fn create_memory_snapshot(&self) -> std::io::Result<()> {
        // Utiliser BrainAPI pour la sauvegarde d'urgence
        self.brain.save_critical_snapshot().await
    }
}

fn main() {
    let mut orchestrator = Orchestrator::new();
    orchestrator.run_autonomous_cycle().unwrap();
}
