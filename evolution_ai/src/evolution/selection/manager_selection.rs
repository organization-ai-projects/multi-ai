use super::{
    population::PopulationUtils,
    strategy::{SelectionMethod, SelectionStrategy},
};

pub struct SelectionManager {
    strategy: SelectionStrategy,
}

impl SelectionManager {
    pub fn new(selection_pressure: f64, min_population: usize, method: SelectionMethod) -> Self {
        Self {
            strategy: SelectionStrategy::new(selection_pressure, min_population, method),
        }
    }

    pub fn select_parents(&self, performers: &[(String, f64)]) -> Option<(String, String)> {
        self.strategy.select_parents(performers)
    }

    /// Évalue un individu et retourne un score de performance.
    pub fn evaluate_individual(&self, individual: &str) -> f64 {
        self.strategy.evaluate_individual(individual)
    }

    /// Évalue une population entière et retourne les performances triées.
    pub fn evaluate_population(&self, population: &[String]) -> Vec<(String, f64)> {
        population
            .iter()
            .map(|individual| {
                let performance = self.evaluate_individual(individual);
                (individual.clone(), performance)
            })
            .collect()
    }

    /// Réduit la population à une taille cible en gardant les meilleurs individus.
    pub fn reduce_population(
        &self,
        performers: &[(String, f64)],
        target_size: usize,
    ) -> Vec<String> {
        PopulationUtils::reduce_population(performers, target_size)
    }

    /// Ajoute de nouveaux individus aléatoires pour diversifier la population.
    pub fn add_diversity<F>(
        &self,
        population: &mut Vec<String>,
        num_new_individuals: usize,
        generator: F,
    ) where
        F: Fn() -> String,
    {
        PopulationUtils::add_diversity(population, num_new_individuals, generator);
    }
}
