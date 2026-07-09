pub struct ResourceManager {
    energy: f64,
    water: f64,
    food: f64,
    regeneration_rate: f64, // Pourcentage de régénération par cycle
}

impl ResourceManager {
    pub fn new(energy: f64, water: f64, food: f64, regeneration_rate: f64) -> Self {
        Self {
            energy,
            water,
            food,
            regeneration_rate,
        }
    }

    /// Régénère les ressources en fonction du taux de régénération.
    pub fn regenerate(&mut self) {
        self.energy += self.energy * self.regeneration_rate;
        self.water += self.water * self.regeneration_rate;
        self.food += self.food * self.regeneration_rate;
    }

    /// Consomme des ressources si elles sont disponibles.
    pub fn consume(&mut self, energy: f64, water: f64, food: f64) -> bool {
        if self.energy >= energy && self.water >= water && self.food >= food {
            self.energy -= energy;
            self.water -= water;
            self.food -= food;
            true
        } else {
            false
        }
    }

    /// Retourne les niveaux actuels des ressources.
    pub fn get_levels(&self) -> (f64, f64, f64) {
        (self.energy, self.water, self.food)
    }
}
