use super::super::types::{MutationProvider, MutationResult};
use rand::{prelude::SliceRandom, Rng};

#[derive(Clone)]
pub struct SafeMutator {
    mutation_rate: f64,
    patterns: Vec<(&'static str, &'static str)>,
}

impl SafeMutator {
    pub fn new(rate: f64) -> Self {
        Self {
            mutation_rate: rate,
            patterns: vec![
                ("+ 1.0", "* 1.1"),
                ("- 1", "+ 1"),
                ("> 0", ">= 0"),
                ("x + 1.0", "x.powf(2.0)"),
                ("x * -1.0", "x.sin()"),
            ],
        }
    }
}

impl MutationProvider for SafeMutator {
    fn mutate(&self, code: &str, max_mutations: usize) -> MutationResult {
        let mut rng = rand::thread_rng();
        let mut mutated = code.to_string();
        let mut mutations_applied = 0;

        while mutations_applied < max_mutations && rng.gen_bool(self.mutation_rate) {
            if let Some((pattern, replacement)) = self.patterns.choose(&mut rng) {
                if mutated.contains(pattern) {
                    mutated = mutated.replacen(pattern, replacement, 1);
                    mutations_applied += 1;
                }
            }
        }

        MutationResult {
            code: mutated,
            success: mutations_applied > 0,
            mutations_applied,
        }
    }

    fn get_mutation_rate(&self) -> f64 {
        self.mutation_rate
    }

    fn set_mutation_rate(&mut self, rate: f64) {
        self.mutation_rate = rate.clamp(0.0, 1.0);
    }

    fn clone_box(&self) -> Box<dyn MutationProvider> {
        Box::new(self.clone())
    }
}
