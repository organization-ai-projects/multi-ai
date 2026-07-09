/// Ce fichier gère une population de stratégies (`Population`).
/// Rôle : Implémenter des mécanismes d'évolution comme la sélection, le croisement et la mutation pour optimiser des stratégies.

use rand::{seq::SliceRandom, thread_rng};
use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde_json;
use std::fs::File;
use std::io::{Read, Write};

// Population de stratégies (ou d'agents complets)
#[derive(Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Population {
    pub individuals: Vec<Strategy>,
    pub history: Vec<Strategy>, // Pour garder mémoire des stratégies passées
}

impl Population {
    pub fn new(size: usize) -> Self {
        let mut individuals = Vec::with_capacity(size);
        for _ in 0..size {
            individuals.push(Strategy {
                pipeline: (0..3).map(|_| Primitive::random()).collect(),
                fitness: 0.0,
                ancestry: vec![],
            });
        }
        Self {
            individuals,
            history: vec![],
        }
    }

    pub fn evaluate(&mut self, fitness_fn: &dyn Fn(&Strategy) -> f64) {
        for ind in &mut self.individuals {
            ind.fitness = fitness_fn(ind);
        }
    }

    pub fn select(&self, survivor_ratio: f64) -> Vec<Strategy> {
        let survivors = (self.individuals.len() as f64 * survivor_ratio) as usize;
        let mut sorted = self.individuals.clone();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        sorted.into_iter().take(survivors).collect()
    }

    pub fn crossover(&self, parent1: &Strategy, parent2: &Strategy) -> Strategy {
        let mut rng = rand::rng();
        let mut pipeline = vec![];
        let min_len = usize::min(parent1.pipeline.len(), parent2.pipeline.len());
        for i in 0..min_len {
            if rng.random_bool(0.5) {
                pipeline.push(parent1.pipeline[i].clone());
            } else {
                pipeline.push(parent2.pipeline[i].clone());
            }
        }
        Strategy {
            pipeline,
            fitness: 0.0,
            ancestry: vec![
                parent1.ancestry.last().unwrap_or(&"".to_string()).clone(),
                parent2.ancestry.last().unwrap_or(&"".to_string()).clone(),
            ],
        }
    }

    pub fn next_generation(&mut self, survivor_ratio: f64, mutation_rate: f64) {
        let survivors = self.select(survivor_ratio);
        let mut rng = rng();
        let mut new_individuals = survivors.clone();

        // Générer de nouveaux individus par crossover + mutation
        while new_individuals.len() < self.individuals.len() {
            let parents = survivors.choose_multiple(&mut rng, 2).collect::<Vec<_>>();
            if parents.len() == 2 {
                let mut child = self.crossover(parents[0], parents[1]);
                if rng.random_bool(mutation_rate) {
                    child = child.mutate();
                }
                new_individuals.push(child);
            }
        }

        // Historique
        self.history.extend(self.individuals.clone());

        self.individuals = new_individuals;
    }

    // Sauvegarde et chargement au format bincode
    pub fn save_bincode(&self, path: &str) -> std::io::Result<()> {
        let encoded = bincode_next::encode_to_vec(self, bincode_next::config::standard()).unwrap();
        let mut file = File::create(path)?;
        file.write_all(&encoded)?;
        Ok(())
    }

    pub fn load_bincode(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        Ok(bincode_next::decode_from_slice(&buf, bincode_next::config::standard()).map(|(v, _)| v).unwrap())
    }

    // Sauvegarde et chargement au format RON
    pub fn save_ron(&self, path: &str) -> std::io::Result<()> {
        let pretty = PrettyConfig::default();
        let ron_str = to_string_pretty(self, pretty).unwrap();
        std::fs::write(path, ron_str)?;
        Ok(())
    }

    pub fn load_ron(path: &str) -> std::io::Result<Self> {
        let ron_str = std::fs::read_to_string(path)?;
        Ok(from_str(&ron_str).unwrap())
    }

    // Sauvegarde et chargement au format JSON
    pub fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_json(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json).unwrap())
    }
}
