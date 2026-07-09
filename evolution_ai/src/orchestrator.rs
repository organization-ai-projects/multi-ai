use crate::environment::catastrophes::CatastropheManager;
use crate::environment::climate::ClimateManager;
use crate::environment::manager_environment::EnvironmentManager;
use crate::evolution::manager_evolution::EvolutionManager;
use crate::system::manager_system::SystemManager;

pub struct Orchestrator {
    system: SystemManager,
    evolution: EvolutionManager,
    catastrophe_manager: CatastropheManager,
    climate_manager: ClimateManager,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            system: SystemManager::new(),
            evolution: EvolutionManager::new(),
            catastrophe_manager: CatastropheManager::new(),
            climate_manager: ClimateManager::new(20.0, 50.0),
        }
    }

    pub fn run_evolution_cycle(&mut self) {
        // 1. Régénérer les ressources de l'environnement
        self.environment.regenerate_resources();

        // 2. Simuler un changement climatique
        self.climate_manager.simulate_climate_change();
        let (temperature, humidity, is_raining) = self.climate_manager.get_conditions();
        println!(
            "Climat actuel : Température = {}, Humidité = {}, Pluie = {}",
            temperature,
            humidity,
            if is_raining { "Oui" } else { "Non" }
        );

        // 3. Fournir des ressources aux IA en fonction de leurs besoins
        let mut population = self.system.get_population();
        self.environment.provide_resources(&mut population);

        // 4. Filtrer les IA avec trop peu d'énergie
        population.retain(|ia| self.environment.get_energy(ia) > 10.0);

        // 5. Évaluer et évoluer la population
        let performers = self.evolution.evaluate_population(&population);
        let new_population = self.evolution.evolve_population(&performers);

        // 6. Appliquer les changements au système
        self.system.update_population(new_population);
    }
}
