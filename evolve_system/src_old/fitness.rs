use rand::Rng;
use std::sync::Arc; // Import du trait `Rng` pour utiliser `gen_bool` et `gen_range`

pub trait Apply {
    fn apply(&self, input: f64) -> f64;
}

#[derive(Clone)]
pub struct FitnessRule<T: Apply> {
    pub rule: Arc<dyn Fn(&T) -> f64 + Send + Sync>, // Utilisation de `Arc` pour rendre clonable
    pub description: String,
}

pub struct FitnessEngine<T: Apply> {
    pub rules: Vec<FitnessRule<T>>,
}

impl<T: Apply> FitnessEngine<T> {
    pub fn new() -> Self {
        // Démarre avec quelques règles simples
        Self {
            rules: vec![FitnessRule {
                rule: Arc::new(|s: &T| {
                    // Ex: “fitness” = précision pour transformer 5.0 en 100.0
                    let out = s.apply(5.0);
                    1.0 / (1.0 + (out - 100.0).abs())
                }),
                description: "Proximité à 100.0 (input=5.0)".to_string(),
            }],
        }
    }

    pub fn add_rule<F>(&mut self, description: &str, rule: F)
    where
        F: Fn(&T) -> f64 + Send + Sync + 'static,
    {
        self.rules.push(FitnessRule {
            rule: Arc::new(rule),
            description: description.to_string(),
        });
    }

    pub fn evaluate(&self, entity: &T) -> f64 {
        // Multi-règles, par exemple moyenne
        if self.rules.is_empty() {
            0.0
        } else {
            self.rules.iter().map(|r| (r.rule)(entity)).sum::<f64>() / self.rules.len() as f64
        }
    }

    pub fn mutate_rule<F>(&mut self, description: &str, rule: F)
    where
        F: Fn(&T) -> f64 + Send + Sync + 'static,
    {
        let mut rng = rand::rng();
        // Ajouter une nouvelle règle aléatoire ou muter une existante
        if rng.random_bool(0.5) && !self.rules.is_empty() {
            // Muter une existante
            let idx = rng.random_range(0..self.rules.len());
            let delta = rng.random_range(-50.0..50.0);
            let new_rule = FitnessRule {
                rule: Arc::new(move |s: &T| {
                    let out = s.apply(5.0 + delta);
                    1.0 / (1.0 + (out - 100.0).abs())
                }),
                description: format!("Proximité à 100.0 (input={})", 5.0 + delta),
            };
            self.rules[idx] = new_rule;
        } else {
            // Nouvelle règle
            let delta = rng.random_range(-100.0..100.0);
            self.rules.push(FitnessRule {
                rule: Arc::new(move |s: &T| {
                    let out = s.apply(3.0 + delta);
                    (out - 7.0).abs().recip()
                }),
                description: format!("Proximité à 7.0 (input={})", 3.0 + delta),
            });
        }
    }
}
