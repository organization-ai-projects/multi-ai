pub struct Temperature {
    pub value: f64,
    pub min: f64,
    pub max: f64,
    pub extreme_threshold: f64, // Seuil pour des températures extrêmes
    pub rain_min: f64,          // Température minimale pour la pluie
    pub rain_max: f64,          // Température maximale pour la pluie
}

impl Temperature {
    pub fn new(initial: f64, min: f64, max: f64) -> Self {
        Self {
            value: initial.clamp(min, max),
            min,
            max,
            extreme_threshold: 40.0, // Par défaut, température extrême au-dessus de 40°C
            rain_min: 0.0,           // Par défaut, pluie possible au-dessus de 0°C
            rain_max: 35.0,          // Par défaut, pluie possible en dessous de 35°C
        }
    }

    /// Met à jour la température en respectant les contraintes.
    pub fn update(&mut self, delta: f64) {
        self.value = (self.value + delta).clamp(self.min, self.max);
    }

    /// Vérifie si la température est extrême.
    pub fn is_extreme(&self) -> bool {
        self.value >= self.extreme_threshold || self.value <= self.min + 5.0
    }

    /// Vérifie si la température est propice à la pluie.
    pub fn is_rain_possible(&self) -> bool {
        self.value >= self.rain_min && self.value <= self.rain_max
    }
}
