mod ai;
mod cli;
mod cli_commands;

fn main() {
    let project_base = "rust_universal";
    let cli = cli::Cli::new(project_base);

    // Comportement par défaut : exécuter le cycle du cerveau
    //let config = BrainCycleConfig::default();
    //run_brain_cycle_with_config(config);
}
