use crate::memory::Memory;
use rand::seq::SliceRandom;

pub struct Mutation {
    pub original_molecule: String,     // Molécule d'origine
    pub mutated_molecule: String,      // Nouvelle molécule après mutation
    pub mutation_type: MutationType,   // Type de mutation appliquée
    pub origin_context: String,        // Contexte où la molécule originale a été vue
}

#[derive(Debug)]
pub enum MutationType {
    Substitution,    // Remplacement d'une partie de la molécule
    Truncation,      // Raccourcissement
    Extension,       // Ajout de caractères
    Fusion,          // Fusion de deux molécules
}

impl Mutation {
    // Tente une mutation sur une molécule
    pub fn try_mutate(molecule: &str, context: &str) -> Option<Self> {
        let mutation_type = Self::random_mutation_type();
        let mutated = match mutation_type {
            MutationType::Substitution => Self::substitute(molecule),
            MutationType::Truncation => Self::truncate(molecule),
            MutationType::Extension => Self::extend(molecule),
            MutationType::Fusion => return None, // Nécessite 2 molécules
        }?;

        Some(Self {
            original_molecule: molecule.to_string(),
            mutated_molecule: mutated,
            mutation_type,
            origin_context: context.to_string(),
        })
    }

    // Tente une fusion entre deux molécules
    pub fn try_fusion(mol1: &str, mol2: &str, context: &str) -> Option<Self> {
        Some(Self {
            original_molecule: format!("{} + {}", mol1, mol2),
            mutated_molecule: format!("{}{}", mol1, mol2), // Simple concaténation pour l'exemple
            mutation_type: MutationType::Fusion,
            origin_context: context.to_string(),
        })
    }

    fn random_mutation_type() -> MutationType {
        use rand::Rng;
        match rand::thread_rng().gen_range(0..4) {
            0 => MutationType::Substitution,
            1 => MutationType::Truncation,
            2 => MutationType::Extension,
            _ => MutationType::Fusion,
        }
    }

    fn substitute(molecule: &str) -> Option<String> {
        // Exemple: remplace un caractère par un autre
        let chars: Vec<char> = molecule.chars().collect();
        if chars.is_empty() { return None; }
        
        let mut rng = rand::thread_rng();
        let pos = rng.gen_range(0..chars.len());
        let new_char = ['_', 'x', 'n', 't'].choose(&mut rng)?;
        
        let mut new_mol = chars;
        new_mol[pos] = *new_char;
        Some(new_mol.iter().collect())
    }

    fn truncate(molecule: &str) -> Option<String> {
        // Raccourcit la molécule
        if molecule.len() <= 1 { return None; }
        let pos = rand::thread_rng().gen_range(1..molecule.len());
        Some(molecule[0..pos].to_string())
    }

    fn extend(molecule: &str) -> Option<String> {
        // Ajoute des caractères à la molécule
        let suffixes = ["_new", "able", "mut", "_t"];
        suffixes.choose(&mut rand::thread_rng())
            .map(|s| format!("{}{}", molecule, s))
    }
}
