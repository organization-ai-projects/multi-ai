use crate::environment::{FileSystem, Population, Fossilization};
use crate::execution::Executor;
use crate::logger::{NatureLogger, LogEntry};
use crate::life_form::{LifeForm, State};
use crate::process_manager::ProcessManager;
use crate::life_manager::LifeManager;
use crate::survival_rules::SurvivalRules;
use crate::memory::NatureMemory;
use crate::natural_selection::NaturalSelection;
use log::{info, debug};

/// Ecosystem coordonne les aspects globaux de l'environnement naturel.
pub struct Ecosystem {
    fs: FileSystem,
    population: Population,
    fossilization: Fossilization,
    executor: Executor,
    logger: NatureLogger,
    process_manager: ProcessManager,
    life_manager: LifeManager,
    natural_selection: NaturalSelection, // Ajout de NaturalSelection
}

impl Ecosystem {
    pub fn new(base_path: std::path::PathBuf) -> Self {
        let log_path = base_path.join("logs");
        std::fs::create_dir_all(&log_path).ok();

        let survival_rules = SurvivalRules::new();
        let memory = NatureMemory::new();
        let natural_selection = NaturalSelection::new(survival_rules, memory);

        Self {
            fs: FileSystem::new(base_path.clone()),
            population: Population::new(),
            fossilization: Fossilization::new(100),
            executor: Executor::new(std::time::Duration::from_secs(5)),
            logger: NatureLogger::new(log_path),
            process_manager: ProcessManager::new(),
            life_manager: LifeManager::new(),
            natural_selection, // Ajout de NaturalSelection
        }
    }

    pub fn init(&mut self) {
        self.fs.init_structure();
    }

    pub fn process_cycle(&mut self) {
        debug!("Début du cycle écologique");

        // Gestion des processus actifs
        self.process_manager.check_active_processes(&mut self.population, &self.executor);

        // Gestion de la reproduction
        self.life_manager.handle_reproduction(&mut self.population, &self.fs);

        // Suppression des formes mortes
        self.life_manager.remove_dead_forms(&mut self.population);

        // Sauvegarde de l'état
        self.save_state();
        debug!("Fin du cycle écologique");
    }

    pub fn introduce_life_form(&mut self, source_code: String) -> String {
        let id = self.population.track_life_form(source_code, None);
        if let Some(form) = self.population.get_form(&id) {
            self.fs.save_life_form(form);
            self.process_manager.launch_life_form(&id, &self.fs);
        }
        id
    }

    fn save_state(&self) {
        self.fs.save_population(&self.population);
        self.fs.save_fossils(&self.fossilization);
    }

    pub fn get_stats(&self) -> EcosystemStats {
        EcosystemStats {
            living_forms: self.population.count_living(),
            dead_forms: self.population.count_dead(),
            fossils: self.fossilization.count(),
            total_lifetime: self.population.total_lifetime(),
        }
    }

    pub fn get_observable_data(&self) -> Vec<ObservableSpecimen> {
        self.population.get_all_forms()
            .map(|form| ObservableSpecimen {
                id: form.id.clone(),
                state: form.current_state.clone(),
                total_lifetime: form.total_lifetime,
                parent_id: form.parent_id.clone(),
            })
            .collect()
    }

    pub fn get_logs(&self, id: &str, zoom_level: u8) -> Vec<LogEntry> {
        self.logger.get_logs(id, zoom_level)
    }

    pub fn clean_shutdown(&mut self) {
        self.process_manager.clean_shutdown();
        self.save_state();
        self.fs.cleanup();
    }
}

pub struct EcosystemStats {
    pub living_forms: usize,
    pub dead_forms: usize,
    pub fossils: usize,
    pub total_lifetime: u64,
}

/// Structure exposée aux observateurs
#[derive(Clone)]
pub struct ObservableSpecimen {
    pub id: String,
    pub state: State,
    pub total_lifetime: u64,
    pub parent_id: Option<String>,
}
