#[derive(Debug)]
pub struct MutationResult {
    pub code: String,
    pub success: bool,
    pub mutations_applied: usize,
}

pub trait MutationProvider: Send + Sync {
    fn mutate(&self, code: &str, max_mutations: usize) -> MutationResult;
    fn get_mutation_rate(&self) -> f64;
    fn set_mutation_rate(&mut self, rate: f64);
    fn clone_box(&self) -> Box<dyn MutationProvider>;
}

// Implémentation spéciale de Clone pour Box<dyn MutationProvider>
impl Clone for Box<dyn MutationProvider> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Debug)]
pub struct PopulationStats {
    pub diversity: f64,
    pub generations_without_improvement: usize,
}
