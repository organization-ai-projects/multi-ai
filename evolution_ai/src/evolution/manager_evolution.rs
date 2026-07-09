use super::crossover::CrossoverManager;
use super::mutations::MutationManager;
use super::selection::{SelectionManager, SelectionMethod};
use rand::prelude::SliceRandom;

#[derive(Clone)]
pub struct EvolutionManager {
    mutator: MutationManager,
    crossover: CrossoverManager,
    selector: SelectionManager,
}

impl EvolutionManager {
    pub fn new() -> Self {
        Self {
            mutator: MutationManager::new(),
            crossover: CrossoverManager::new(),
            selector: SelectionManager::new(0.7, 10, SelectionMethod::Roulette),
        }
    }

    pub fn evolve_population(&mut self, performers: &[(String, f64)]) -> Vec<String> {
        let mut new_population = Vec::new();

        // 1. Sélection des parents
        while let Some((parent1, parent2)) = self.selector.select_parents(performers) {
            // 2. Crossover entre les parents
            if let Some(child) = self.crossover.try_crossover(&parent1, &parent2) {
                // 3. Mutation de l'enfant
                let mutated_child = self.mutator.mutate_code(&child, "Generalist");
                new_population.push(mutated_child);
            }
        }

        new_population
    }

    /// Évalue une population entière et retourne les performances triées.
    pub fn evaluate_population(&self, population: &[String]) -> Vec<(String, f64)> {
        population
            .iter()
            .map(|individual| {
                let performance = self.selector.evaluate_individual(individual);
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
        performers
            .iter()
            .take(target_size)
            .map(|(individual, _)| individual.clone())
            .collect()
    }

    /// Ajoute de nouveaux individus à la population en utilisant des mutations sur des individus existants.
    pub fn add_diversity(&mut self, population: &mut Vec<String>, num_new_individuals: usize) {
        let mut rng = rand::rng();
        for _ in 0..num_new_individuals {
            if let Some(base_individual) = population.choose(&mut rng) {
                // Utiliser MutationManager pour muter un individu existant
                let new_individual = self.mutator.mutate_code(base_individual, "Generalist");
                population.push(new_individual);
            }
        }
    }

    /// Exécute un cycle complet d'évolution.
    pub fn run_full_evolution_cycle(
        &mut self,
        population: &mut Vec<String>,
        target_size: usize,
    ) -> Vec<String> {
        // 1. Déclencher une catastrophe globale (par exemple, 50% de survie)
        self.trigger_catastrophe(population, 0.5);

        // 2. Évaluer la population
        let mut performers = self.selector.evaluate_population(population);

        // 3. Trier par performance (descendant)
        performers.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // 4. Sélectionner les meilleurs individus pour la reproduction
        let selected_parents = self
            .selector
            .reduce_population(&performers, target_size / 2);

        // 5. Reproduire une nouvelle population via crossover et mutation
        let mut new_population = self.evolve_population(&performers);

        // 6. Réduire la population pour conserver uniquement les meilleurs
        new_population.extend(selected_parents);
        let reduced_population = self.selector.reduce_population(
            &self.selector.evaluate_population(&new_population),
            target_size,
        );

        // 7. Ajouter de la diversité si nécessaire
        self.add_diversity(&mut new_population, target_size - reduced_population.len());

        // Retourner la nouvelle population
        new_population
    }

    /// Déclenche une catastrophe globale qui réduit la population.
    pub fn trigger_catastrophe(&mut self, population: &mut Vec<String>, survival_rate: f64) {
        let mut rng = rand::rng();
        population.retain(|_| rng.random_bool(survival_rate));
    }
}
