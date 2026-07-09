pub struct Humidity {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub critical_threshold: f64, // Seuil critique pour déclencher la pluie
}

impl Humidity {
    pub fn new(initial: f64, min: f64, max: f64) -> Self {
        Self {
            value: initial.clamp(min, max),
            min,
            max,
            critical_threshold: 80.0, // Par défaut, la pluie commence au-dessus de 80%
        }
    }

    /// Met à jour l'humidité en respectant les contraintes.
    pub fn update(&mut self, delta: f64) {
        self.value = (self.value + delta).clamp(self.min, self.max);
    }

    /// Vérifie si l'humidité est suffisante pour permettre la pluie.
    pub fn is_humid_enough(&self) -> bool {
        self.value >= self.critical_threshold
    }

    /// Réduit l'humidité après une pluie.
    pub fn reduce_after_rain(&mut self, reduction: f64) {
        self.value = (self.value - reduction).clamp(self.min, self.max);
    }
}
