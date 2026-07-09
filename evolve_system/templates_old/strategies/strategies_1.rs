// ----- 1. Stratégies : définitions et évolution dynamique -----

use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};

// Représente une primitive (opération/unité de logique de base)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Primitive {
    Add(f64),       // Additionne une constante
    Mul(f64),       // Multiplie par une constante
    Threshold(f64), // Sortie = 1 si entrée > x, sinon 0
    Custom(String), // Pour des opérations synthétisées dynamiquement (ex: du code généré)
                    // Ajoute d'autres primitives selon ton système
}

// Un pipeline (= stratégie candidate)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Strategy {
    pub pipeline: Vec<Primitive>,
    pub fitness: f64,
    pub ancestry: Vec<String>, // Historique (UUID, parent, etc)
}

impl Strategy {
    pub fn apply(&self, input: f64) -> f64 {
        self.pipeline.iter().fold(input, |mut acc, prim| {
            match prim {
                Primitive::Add(x) => acc += x,
                Primitive::Mul(x) => acc *= x,
                Primitive::Threshold(th) => acc = if acc > *th { 1.0 } else { 0.0 },
                Primitive::Custom(code) => {
                    // Ici tu pourrais brancher un système d’interprétation dynamique ou une lib externe
                    acc // placeholder: on ne fait rien
                }
            }
            acc
        })
    }

    pub fn mutate(&self) -> Self {
        let mut rng = rand::thread_rng();
        let mut new_pipeline = self.pipeline.clone();

        // Mutation simple : ajoute/enlève/mute une primitive
        match rng.gen_range(0..4) {
            0 if new_pipeline.len() > 1 => {
                new_pipeline.remove(rng.gen_range(0..new_pipeline.len()));
            }
            1 => {
                // Ajout d'une primitive aléatoire
                new_pipeline.insert(rng.gen_range(0..=new_pipeline.len()), Primitive::random());
            }
            2 => {
                // Mutation d'un paramètre
                if let Some(p) = new_pipeline.choose_mut(&mut rng) {
                    *p = Primitive::random();
                }
            }
            3 => {
                // Synthèse d'une nouvelle primitive (méta-mutation)
                if rng.gen_bool(0.2) {
                    new_pipeline.push(Primitive::Custom("generated_code_xyz".to_string()));
                }
            }
            _ => {}
        }
        let mut ancestry = self.ancestry.clone();
        ancestry.push(uuid::Uuid::new_v4().to_string());
        Strategy {
            pipeline: new_pipeline,
            fitness: 0.0,
            ancestry,
        }
    }
}

impl Primitive {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Primitive::Add(rng.gen_range(-10.0..10.0)),
            1 => Primitive::Mul(rng.gen_range(0.1..3.0)),
            2 => Primitive::Threshold(rng.gen_range(-5.0..5.0)),
            _ => Primitive::Add(1.0),
        }
    }
}
