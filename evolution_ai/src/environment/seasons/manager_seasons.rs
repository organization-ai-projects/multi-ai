#[derive(Debug, Clone, Copy)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

pub struct SeasonManager {
    day_of_year: usize,
    days_per_season: usize,
}

impl SeasonManager {
    pub fn new() -> Self {
        Self {
            day_of_year: 0,
            days_per_season: 365 / 4, // Diviser l'année en 4 saisons égales
        }
    }

    /// Avance d'un jour dans le cycle annuel.
    pub fn advance_day(&mut self) {
        self.day_of_year = (self.day_of_year + 1) % 365;
    }

    /// Retourne la saison actuelle en fonction du jour de l'année.
    pub fn get_current_season(&self) -> Season {
        match self.day_of_year / self.days_per_season {
            0 => Season::Spring,
            1 => Season::Summer,
            2 => Season::Autumn,
            _ => Season::Winter,
        }
    }

    /// Retourne les effets de la saison actuelle sur la température et l'humidité.
    pub fn get_seasonal_effects(&self) -> (f64, f64) {
        match self.get_current_season() {
            Season::Spring => (5.0, 10.0),    // Température +5°C, Humidité +10%
            Season::Summer => (10.0, -5.0),   // Température +10°C, Humidité -5%
            Season::Autumn => (-5.0, 5.0),    // Température -5°C, Humidité +5%
            Season::Winter => (-10.0, -10.0), // Température -10°C, Humidité -10%
        }
    }
}
