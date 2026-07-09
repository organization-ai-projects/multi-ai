use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Species {
    pub name: String,
    pub mutation_brutality: f64, // Entre 0.0 (faible) et 1.0 (très brutal)
    pub mutation_guidance: f64,  // Entre 0.0 (aléatoire) et 1.0 (très guidé)
}

impl Species {
    /// Crée une nouvelle espèce avec des paramètres spécifiques.
    pub fn new(name: &str, mutation_brutality: f64, mutation_guidance: f64) -> Self {
        Self {
            name: name.to_string(),
            mutation_brutality: mutation_brutality.clamp(0.0, 1.0),
            mutation_guidance: mutation_guidance.clamp(0.0, 1.0),
        }
    }

    /// Retourne une description dynamique de l'espèce.
    pub fn description(&self) -> String {
        format!(
            "{}: Brutalité des mutations = {:.2}, Guidage des mutations = {:.2}",
            self.name, self.mutation_brutality, self.mutation_guidance
        )
    }
}
