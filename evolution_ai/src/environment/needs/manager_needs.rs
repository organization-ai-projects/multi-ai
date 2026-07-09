use super::needs_model::{Needs, Resource};
use std::fs;

pub struct NeedsManager;

impl NeedsManager {
    pub fn new() -> Self {
        Self {}
    }

    /// Charge les besoins d'une IA à partir d'un fichier `.ron`.
    pub fn load_needs(&self, ia_path: &str) -> Needs {
        let needs_path = format!("{}/needs.ron", ia_path);
        if let Ok(content) = fs::read_to_string(&needs_path) {
            ron::from_str(&content).unwrap_or_default()
        } else {
            Needs::default()
        }
    }

    /// Calcule les besoins d'une IA en fonction de son code.
    pub fn calculate_needs_from_code(&self, ia_path: &str) -> Needs {
        let mut needs = Needs::default();

        if let Ok(entries) = fs::read_dir(ia_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Exemple : augmenter les besoins en énergie pour des fichiers longs
                        needs.energy += content.len() as f64 * 0.01;

                        // Exemple : réduire les besoins en eau si des structures avancées sont présentes
                        if content.contains("if ") || content.contains("else ") {
                            needs.water -= 1.0;
                        }

                        // Exemple : augmenter les besoins en nourriture pour des pratiques inefficaces
                        if content.contains("unwrap()") {
                            needs.food += 5.0;
                        }
                    }
                }
            }
        }

        needs
    }

    /// Met à jour les besoins d'une IA.
    pub fn update_needs(&self, needs: &mut Needs, resources: &[Resource]) {
        for resource in resources {
            match resource.name.as_str() {
                "energy" => needs.energy -= resource.amount,
                "water" => needs.water -= resource.amount,
                "food" => needs.food -= resource.amount,
                _ => {}
            }
        }
    }
}
