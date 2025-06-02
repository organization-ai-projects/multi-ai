use crate::environment::{FileSystem, Population, Fossilization};
use crate::natural_selection::NaturalSelection;
use crate::execution::Executor;
use crate::life_cycle::Lifecycle;
use crate::life_form::{LifeForm, State};
use std::path::PathBuf;
use std::time::Duration;
use log::{info, warn, debug};

/// Ecosystem coordonne tous les aspects de l'environnement naturel :
/// - Gestion des formes de vie via Population
/// - Processus de sélection naturelle
/// - Cycle de vie et reproduction
/// - Fossilisation des spécimens morts
pub struct Ecosystem {
    fs: FileSystem,
    population: Population,
    fossilization: Fossilization,
    selection: NaturalSelection,
    executor: Executor,
    lifecycle: Lifecycle,
    base_path: PathBuf
}

impl Ecosystem {
    pub fn new(base_path: PathBuf) -> Self {
        Self {
            fs: FileSystem::new(base_path.clone()),
            population: Population::new(),
            fossilization: Fossilization::new(100), // max 100 fossiles
            selection: NaturalSelection::new(),
            executor: Executor::new(Duration::from_secs(5)),
            lifecycle: Lifecycle::new(base_path.clone()),
            base_path
        }
    }

    pub fn init(&mut self) {
        self.fs.init_structure();
    }

    pub fn process_cycle(&mut self) {
        debug!("Début du cycle écologique");

        // 1. Évolution et sélection naturelle
        let living_count = self.population.get_living_forms().count();
        info!("Évaluation de {} formes de vie", living_count);

        // 2. Cycle de reproduction
        if let Some(new_forms) = self.lifecycle.process_reproduction(&self.population) {
            info!("Nouvelles formes de vie créées: {}", new_forms.len());
            for form in new_forms {
                self.population.add_form(form);
            }
        }

        // 3. Nettoyage et fossilisation
        let dead_count = self.population.get_dead_forms().count();
        if dead_count > 0 {
            info!("Traitement de {} formes mortes", dead_count);
            self.process_fossils();
        }

        // 4. Sauvegarde de l'état
        self.save_state();
        debug!("Fin du cycle écologique - {} formes survivantes", 
            self.population.get_living_forms().count());
    }

    fn process_life_form(&mut self, form: &mut LifeForm) {
        let exec_result = self.executor.execute(&self.fs.get_form_path(&form.id));
        let selection_result = self.selection.evaluate(form);
        
        match (exec_result.survived, selection_result.survived) {
            (true, true) => {
                debug!("Forme de vie {} a survécu", form.id);
                form.update_state(exec_result);
            },
            _ => {
                warn!("Forme de vie {} n'a pas survécu", form.id);
                form.current_state = State::Dead;
            }
        }
    }

    // 3. Fossilisation des morts
    fn process_fossils(&mut self) {
        let dead_forms = self.population.get_dead_forms();
        for form in dead_forms {
            if self.fossilization.try_fossilize(form) {
                self.population.remove_form(&form.id);
            }
        }
    }

    pub fn introduce_life_form(&mut self, source_code: String) -> String {
        // Création d'une nouvelle forme de vie
        let id = self.population.track_life_form(source_code, None);
        
        // Sauvegarde physique
        if let Some(form) = self.population.get_form(&id) {
            self.fs.save_life_form(form);
        }

        id
    }

    fn save_state(&self) {
        // Sauvegarde de la population
        for form in self.population.get_all_forms() {
            self.fs.save_life_form(form);
        }

        // Sauvegarde des fossiles
        for fossil in self.fossilization.get_fossils() {
            self.fs.save_fossil(fossil);
        }

        // État global
        self.fs.save_ecosystem_state(
            self.population.count(),
            self.fossilization.count()
        );
    }

    pub fn get_stats(&self) -> EcosystemStats {
        EcosystemStats {
            living_forms: self.population.count_living(),
            dead_forms: self.population.count_dead(),
            fossils: self.fossilization.count(),
            total_lifetime: self.population.total_lifetime(),
        }
    }
}

pub struct EcosystemStats {
    pub living_forms: usize,
    pub dead_forms: usize,
    pub fossils: usize,
    pub total_lifetime: u64,
}
