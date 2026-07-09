use systemstat::{Platform, System as StatSystem};

mod detection;
mod windows;

// Re-export des fonctionnalités utilisées à l'extérieur
pub use detection::get_max_temperature;
pub use windows::try_get_temperature_windows;

/// Analyse un niveau de température et retourne une description du risque
pub fn analyze_temperature_risk(temp: f32) -> TempRiskLevel {
    if temp > 90.0 {
        TempRiskLevel::Critical
    } else if temp > 80.0 {
        TempRiskLevel::High
    } else if temp > 70.0 {
        TempRiskLevel::Medium
    } else {
        TempRiskLevel::Normal
    }
}

/// Niveaux de risque pour la température
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TempRiskLevel {
    Normal,
    Medium,
    High,
    Critical,
}
