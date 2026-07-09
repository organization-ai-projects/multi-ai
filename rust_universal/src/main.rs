use rust_universal::ai::manager_ai::{create_agent::create_agent, launch_all};
use rust_universal::ai::orchestrator::brain_orchestrator::{run_brain_cycle_with_config, BrainCycleConfig};
use rust_universal::cli;

fn main() {
    let project_base = "rust_universal";
    let cli = cli::Cli::new(project_base);

    // Comportement par défaut : exécuter le cycle du cerveau
    //let config = BrainCycleConfig::default();
    //run_brain_cycle_with_config(config);
}
