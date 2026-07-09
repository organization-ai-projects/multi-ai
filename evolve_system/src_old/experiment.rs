/// Ce fichier définit la structure `Experiment` et ses méthodes associées.
/// Rôle : Orchestrer les interactions entre `Explorer`, `Strategy`, et `FitnessEngine` pour exécuter une expérimentation complète.
use crate::explorer::Explorer;
use crate::fitness::FitnessEngine;
use crate::primitive::Primitive;
use crate::strategies::Strategy;
use std::collections::HashMap;

pub struct Experiment {
    pub input: String,
    pub explorer: Explorer,
    pub strategies: Vec<Strategy>,
    pub custom_ops: HashMap<String, Box<dyn Fn(f64) -> f64>>, // Opérations dynamiques
    pub fitness_engine: FitnessEngine<Strategy>, // Moteur de fitness générique pour `Strategy`
}

impl Experiment {
    /// Crée une nouvelle expérimentation avec un moteur de fitness par défaut
    pub fn new_with_default_fitness(input: &str) -> Self {
        let mut fitness_engine = FitnessEngine::new();
        fitness_engine.add_rule("Proximité à 100.0 (input=5.0)", |s: &Strategy| {
            let out = s.apply(5.0, &HashMap::new());
            1.0 / (1.0 + (out - 100.0).abs())
        });

        Self {
            input: input.to_string(),
            explorer: Explorer::new(),
            strategies: Vec::new(),
            custom_ops: HashMap::new(),
            fitness_engine,
        }
    }

    /// Ajoute une stratégie en spécifiant une liste de primitives
    pub fn add_strategy_from_primitives(&mut self, primitives: Vec<Primitive>) {
        self.strategies.push(Strategy {
            pipeline: primitives,
            ancestry: vec![],
        });
    }

    /// Ajoute une stratégie en spécifiant une liste de primitives par leur nom
    pub fn add_strategy_from_pipeline(&mut self, pipeline: Vec<&str>) {
        let primitives = pipeline
            .into_iter()
            .map(|name| Primitive::Custom(name.to_string()))
            .collect();
        self.add_strategy_from_primitives(primitives);
    }

    /// Ajoute une opération dynamique
    pub fn add_custom_op(&mut self, name: &str, op: Box<dyn Fn(f64) -> f64>) {
        self.custom_ops.insert(name.to_string(), op);
    }

    /// Orchestration : Exécute les stratégies et les explorations
    pub fn run(
        &mut self,
        transformations: &[Box<dyn Fn(&str) -> String + Send + Sync>],
    ) -> (Vec<f64>, Vec<String>, Option<String>) {
        // Utilise `FitnessEngine` pour évaluer les stratégies
        for strategy in &mut self.strategies {
            strategy.fitness = self.fitness_engine.evaluate(strategy);
        }

        // Filtrer les stratégies basées sur leur fitness
        self.strategies.retain(|s| s.fitness > 0.0); // Exemple : garder uniquement les stratégies valides

        // Utilise `Explorer` pour appliquer toutes les transformations
        let exploration_results = self.explorer.try_all(&self.input, transformations);

        // Utilise `Explorer` pour appliquer une transformation aléatoire
        let random_result = self
            .explorer
            .try_random_action(&self.input, transformations);

        // Retourne les résultats combinés
        let strategy_results: Vec<f64> = self.strategies.iter().map(|s| s.fitness).collect();
        (strategy_results, exploration_results, Some(random_result))
    }

    /// Orchestration : Crée une transformation composée et l'applique
    pub fn run_meta_explore(
        &self,
        transformations: &[Box<dyn Fn(&str) -> String + Send + Sync>],
    ) -> Option<String> {
        if let Some(composed_transformation) = self.explorer.meta_explore(transformations) {
            Some(composed_transformation(&self.input))
        } else {
            None
        }
    }
}
