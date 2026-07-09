use chrono::{DateTime, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

use crate::ai_manager::AIManager;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SpeciesType {
    ByteMutator, // Mutation au niveau bytes
    AstMutator,  // Mutation structurelle (AST)
    SafeMutator, // Mutations contrôlées
    Generalist,  // Mix des stratégies
}

impl SpeciesType {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Self::ByteMutator,
            1 => Self::AstMutator,
            2 => Self::SafeMutator,
            _ => Self::Generalist,
        }
    }

    pub fn mutation_config(&self) -> MutationConfig {
        match self {
            Self::ByteMutator => MutationConfig {
                max_mutations: 5,
                block_size: 1..5,
                rollback_on_error: true,
            },
            Self::AstMutator => MutationConfig {
                max_mutations: 2,
                block_size: 10..50,
                rollback_on_error: true,
            },
            Self::SafeMutator => MutationConfig {
                max_mutations: 10,
                block_size: 1..1,
                rollback_on_error: true,
            },
            Self::Generalist => MutationConfig {
                max_mutations: 3,
                block_size: 1..10,
                rollback_on_error: true,
            },
        }
    }

    pub fn handle_extinction(&self, stats: &SpeciesStats) -> ExtinctionAction {
        let extinct_too_soon = stats.total_generations < 10;
        let was_performing = stats.peak_performance > 0.5;

        if extinct_too_soon || was_performing {
            ExtinctionAction::ReviveFromAncestor
        } else {
            ExtinctionAction::AllowExtinction
        }
    }

    pub fn should_crossover_interspecies(&self, other: &SpeciesType) -> bool {
        let mut rng = rand::thread_rng();
        // Très rare crossover inter-espèces (0.1%)
        rng.gen_bool(0.001)
    }
}

#[derive(Clone)] // Ajout du trait Clone
pub struct MutationConfig {
    pub max_mutations: usize,
    pub block_size: std::ops::Range<usize>,
    pub rollback_on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lineage {
    pub species: SpeciesType,
    pub ancestor_id: String,
    pub generation_created: usize,
    pub mutations_survived: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesStats {
    pub total_generations: usize,
    pub peak_performance: f64,
    pub last_seen: DateTime<Utc>,
    pub population_size: usize,
    pub mutation_rate_boost: f64,
}

impl SpeciesStats {
    pub fn update_from_performers(
        stats: &mut std::collections::HashMap<SpeciesType, SpeciesStats>,
        performers: &[(String, f64)],
        manager: &AIManager,
    ) {
        for (path, score) in performers {
            if let Some(lineage) = manager.get_lineage(path) {
                let species_stat = stats
                    .entry(lineage.species.clone())
                    .or_insert(SpeciesStats {
                        total_generations: 0,
                        peak_performance: 0.0,
                        last_seen: Utc::now(),
                        population_size: 0,
                        mutation_rate_boost: 1.0,
                    });

                species_stat.population_size += 1;
                species_stat.peak_performance = species_stat.peak_performance.max(*score);
                species_stat.last_seen = Utc::now();
            }
        }
    }
}

pub enum ExtinctionAction {
    ReviveFromAncestor,
    AllowExtinction,
}
