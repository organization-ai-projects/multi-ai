use super::super::types::{MutationProvider, MutationResult};
use super::{AstMutator, ByteMutator, GeneralistMutator, SafeMutator};

pub struct StrategiesManager {
    ast_mutator: AstMutator,
    byte_mutator: ByteMutator,
    safe_mutator: SafeMutator,
    generalist_mutator: GeneralistMutator,
}

impl StrategiesManager {
    pub fn new(mutation_rate: f64) -> Self {
        Self {
            ast_mutator: AstMutator::new(mutation_rate),
            byte_mutator: ByteMutator::new(mutation_rate),
            safe_mutator: SafeMutator::new(mutation_rate),
            generalist_mutator: GeneralistMutator::new(mutation_rate),
        }
    }

    pub fn mutate(&mut self, code: &str, strategy: &str, max_mutations: usize) -> MutationResult {
        let mutator: &mut dyn MutationProvider = match strategy {
            "AstMutator" => &mut self.ast_mutator,
            "ByteMutator" => &mut self.byte_mutator,
            "SafeMutator" => &mut self.safe_mutator,
            _ => &mut self.generalist_mutator,
        };

        mutator.mutate(code, max_mutations)
    }
}
