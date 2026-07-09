#[derive(Clone)]
pub struct FitnessRule {
    pub rule: Box<dyn Fn(&Strategy) -> f64 + Send + Sync>,
    pub description: String,
}

pub struct FitnessEngine {
    pub rules: Vec<FitnessRule>,
}

impl FitnessEngine {
    pub fn new() -> Self {
        // Démarre avec quelques règles simples
        Self {
            rules: vec![FitnessRule {
                rule: Box::new(|s: &Strategy| {
                    // Ex: “fitness” = précision pour transformer 5.0 en 100.0
                    let out = s.apply(5.0);
                    1.0 / (1.0 + (out - 100.0).abs())
                }),
                description: "Proximité à 100.0 (input=5.0)".to_string(),
            }],
        }
    }

    pub fn evaluate(&self, strategy: &Strategy) -> f64 {
        // Multi-règles, par exemple moyenne
        self.rules.iter().map(|r| (r.rule)(strategy)).sum::<f64>() / self.rules.len() as f64
    }

    pub fn mutate_rule(&mut self) {
        let mut rng = rand::rng();
        // Ajouter une nouvelle règle aléatoire ou muter une existante
        if rng.random_bool(0.5) && !self.rules.is_empty() {
            // Muter une existante
            let idx = rng.random_range(0..self.rules.len());
            let delta = rng.random_range(-50.0..50.0);
            let new_rule = FitnessRule {
                rule: Box::new(move |s: &Strategy| {
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
                rule: Box::new(move |s: &Strategy| {
                    let out = s.apply(3.0 + delta);
                    (out - 7.0).abs().recip()
                }),
                description: format!("Proximité à 7.0 (input={})", 3.0 + delta),
            });
        }
    }
}
