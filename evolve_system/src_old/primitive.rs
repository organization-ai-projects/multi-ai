/// Ce fichier définit des primitives pour manipuler des données.
/// Rôle : Fournir des opérations primitives (`Primitive`) pour transformer des données.
use rand::{seq::SliceRandom, Rng};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Les opérations primitives que la stratégie peut enchaîner
#[derive(Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub enum Primitive {
    Add(f64),
    Mul(f64),
    Threshold(f64),
    Custom(String), // Permet d'ajouter des primitives dynamiques
}

impl Primitive {
    pub fn apply(&self, input: f64, custom_ops: &HashMap<String, Box<dyn Fn(f64) -> f64>>) -> f64 {
        match self {
            Primitive::Add(x) => input + x,
            Primitive::Mul(x) => input * x,
            Primitive::Threshold(th) => {
                if input > *th {
                    1.0
                } else {
                    0.0
                }
            }
            Primitive::Custom(name) => {
                if let Some(op) = custom_ops.get(name) {
                    op(input)
                } else {
                    input // Si l'opération n'est pas trouvée, retourne l'entrée inchangée
                }
            }
        }
    }

    pub fn random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Primitive::Add(rng.random_range(-10.0..10.0)),
            1 => Primitive::Mul(rng.random_range(0.1..3.0)),
            2 => Primitive::Threshold(rng.random_range(-5.0..5.0)),
            3 => Primitive::Custom("dynamic_op".to_string()), // Exemple de primitive dynamique
            _ => Primitive::Add(1.0),
        }
    }
}
