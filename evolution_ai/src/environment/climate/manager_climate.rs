use super::{Humidity, Temperature};
use rand::Rng;

pub struct ClimateManager {
    temperature: Temperature,
    humidity: Humidity,
    is_raining: bool,
    day_of_year: usize, // Pour gérer la saisonnalité
}

impl ClimateManager {
    pub fn new(initial_temp: f64, initial_humidity: f64) -> Self {
        Self {
            temperature: Temperature::new(initial_temp, -50.0, 50.0),
            humidity: Humidity::new(initial_humidity, 0.0, 100.0),
            is_raining: false,
            day_of_year: 0,
        }
    }

    /// Simule un changement climatique avec saisonnalité, événements rares et pluie variable.
    pub fn simulate_climate_change(&mut self) {
        let mut rng = rand::rng();

        // 1. Appliquer la saisonnalité (sinusoïde lente)
        let seasonal_temp_variation =
            10.0 * (2.0 * std::f64::consts::PI * self.day_of_year as f64 / 365.0).sin();
        let seasonal_humidity_variation =
            5.0 * (2.0 * std::f64::consts::PI * self.day_of_year as f64 / 365.0).cos();
        self.temperature.update(seasonal_temp_variation);
        self.humidity.update(seasonal_humidity_variation);

        // 2. Déclencher des événements rares
        if rng.random_bool(0.01) {
            self.trigger_rare_event();
        }

        // 3. Fluctuations normales
        let temp_delta = rng.random_range(-2.0..2.0);
        let humidity_delta = rng.random_range(-5.0..5.0);
        self.temperature.update(temp_delta);
        self.humidity.update(humidity_delta);

        // 4. Gestion de la pluie avec quantité variable
        if self.should_start_rain() && !self.is_raining {
            self.is_raining = true;
            let rain_amount = self.calculate_rain_amount();
            println!(
                "La pluie commence avec une intensité de {:.2} mm !",
                rain_amount
            );
        } else if self.is_raining && !self.should_continue_rain() {
            self.is_raining = false;
            println!("La pluie s'arrête.");
            self.humidity.reduce_after_rain(10.0); // Réduction de l'humidité après la pluie
        }

        // Avancer dans le cycle annuel
        self.day_of_year = (self.day_of_year + 1) % 365;
    }

    /// Déclenche un événement rare.
    fn trigger_rare_event(&mut self) {
        let mut rng = rand::rng();
        match rng.random_range(0..2) {
            0 => {
                println!("Sécheresse ! Réduction drastique de l'humidité et de l'eau.");
                self.humidity.update(-20.0);
            }
            1 => {
                println!("Tempête ! Augmentation de l'humidité mais réduction des ressources alimentaires.");
                self.humidity.update(30.0);
            }
            _ => {}
        }
    }

    /// Calcule la quantité de pluie en fonction de l'excès d'humidité.
    fn calculate_rain_amount(&self) -> f64 {
        let excess_humidity = self.humidity.value - self.humidity.critical_threshold;
        if excess_humidity > 0.0 {
            excess_humidity * 2.0 // Exemple : chaque % d'excès génère 2 mm de pluie
        } else {
            0.0
        }
    }

    /// Vérifie si la pluie doit commencer.
    fn should_start_rain(&self) -> bool {
        self.humidity.is_humid_enough() && self.temperature.is_rain_possible()
    }

    /// Vérifie si la pluie doit continuer.
    fn should_continue_rain(&self) -> bool {
        self.humidity.value > 60.0 // Exemple : la pluie continue si l'humidité reste au-dessus de 60%
    }

    /// Retourne les conditions climatiques actuelles.
    pub fn get_conditions(&self) -> (f64, f64, bool) {
        (self.temperature.value, self.humidity.value, self.is_raining)
    }

    /// Applique les effets saisonniers à la température et à l'humidité.
    pub fn apply_seasonal_effects(&mut self, temp_effect: f64, humidity_effect: f64) {
        self.temperature.update(temp_effect);
        self.humidity.update(humidity_effect);
    }
}
