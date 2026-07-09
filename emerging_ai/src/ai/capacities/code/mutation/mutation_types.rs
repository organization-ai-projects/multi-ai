use syn::{Expr, ItemFn};

#[derive(Debug, Clone)]
pub enum MutationIntensity {
    Micro,   // Petits changements (caractères, tokens)
    Minor,   // Changements légers (expressions)
    Major,   // Gros changements (blocs entiers)
    Radical, // Mutation complète du code
}

#[derive(Debug, Clone)]
pub struct MutationResult {
    pub original_code: String,
    pub mutated_code: String,
    pub intensity: MutationIntensity,
    pub mutation_points: Vec<usize>, // Où la mutation a eu lieu
}
