use super::{GlobalCatastrophe, LocalCatastrophe};

pub struct CatastropheManager {
    global: GlobalCatastrophe,
    local: LocalCatastrophe,
}

impl CatastropheManager {
    pub fn new() -> Self {
        Self {
            global: GlobalCatastrophe,
            local: LocalCatastrophe,
        }
    }

    /// Déclenche une catastrophe globale qui réduit la population.
    pub fn trigger_global(&self, population: &mut Vec<String>, survival_rate: f64) {
        self.global.trigger(population, survival_rate);
    }

    /// Déclenche une catastrophe locale qui affecte une partie spécifique de la population.
    pub fn trigger_local<F>(&self, population: &mut Vec<String>, filter: F)
    where
        F: Fn(&String) -> bool,
    {
        self.local.trigger(population, filter);
    }
}
