use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Experiment {
    pub input: String,
    pub result: String,
    pub success: bool,
}

pub struct Explorer {
    pub history: Vec<Experiment>,
    pub strategies: Vec<Box<dyn Fn(&str) -> String + Send + Sync>>,
}

impl Explorer {
    pub fn new() -> Self {
        // Ajoute ici autant de stratégies que tu veux (simples ou folles)
        Self {
            history: Vec::new(),
            strategies: vec![
                Box::new(|input| format!("{}!", input)), // Ajoute un !
                Box::new(|input| input.chars().rev().collect()), // Inverse la string
                Box::new(|input| input.to_uppercase()),
                Box::new(|input| format!("{}{}", input, input)),
                // ... d’autres mutations random
            ],
        }
    }

    pub fn try_random_action(&mut self, input: &str) -> &Experiment {
        let mut rng = rand::rng();
        let strat = self.strategies.choose(&mut rng).unwrap();
        let result = strat(input);

        // Simule le succès (par ex, succès si plus long que l’input original)
        let success = result.len() > input.len();

        let exp = Experiment {
            input: input.to_string(),
            result,
            success,
        };
        self.history.push(exp);
        self.history.last().unwrap()
    }

    pub fn try_all(&mut self, input: &str) {
        for strat in &self.strategies {
            let result = strat(input);
            let success = result.len() > input.len(); // Change ici le critère
            self.history.push(Experiment {
                input: input.to_string(),
                result,
                success,
            });
        }
    }

    /// Sélectionne les expériences “réussies”
    pub fn best_results(&self) -> Vec<&Experiment> {
        self.history.iter().filter(|e| e.success).collect()
    }

    /// Essaie tout ce qu’elle connaît sur une population d’inputs
    pub fn explore_population(&mut self, inputs: &[&str]) {
        for input in inputs {
            self.try_all(input);
        }
    }
}
