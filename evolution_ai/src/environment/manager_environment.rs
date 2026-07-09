use super::catastrophes::CatastropheManager;
use super::climate::ClimateManager;
use super::needs::{Needs, NeedsManager};
use super::resources::ResourceManager;
use super::seasons::SeasonManager; // Ajout du SeasonManager
use rand::Rng;
use std::collections::HashMap;
use std::fs;

pub struct EnvironmentManager {
    energy_levels: HashMap<String, f64>, // Associe une IA à son niveau d'énergie
    catastrophe_manager: CatastropheManager,
    climate_manager: ClimateManager,
    needs_manager: NeedsManager,
    resource_manager: ResourceManager, // Remplacement par le ResourceManager
    season_manager: SeasonManager,     // Gestionnaire des saisons
}

impl EnvironmentManager {
    pub fn new() -> Self {
        Self {
            energy_levels: HashMap::new(),
            catastrophe_manager: CatastropheManager::new(),
            climate_manager: ClimateManager::new(20.0, 50.0),
            needs_manager: NeedsManager::new(),
            resource_manager: ResourceManager::new(1000.0, 500.0, 300.0, 0.1), // Exemple de valeurs initiales
            season_manager: SeasonManager::new(),
        }
    }

    /// Initialise l'énergie pour une population d'IA.
    pub fn initialize_energy(&mut self, population: &[String], initial_energy: f64) {
        for ia in population {
            self.energy_levels.insert(ia.clone(), initial_energy);
        }
    }

    /// Applique des pénalités ou des bonus en fonction des caractéristiques des IA.
    pub fn apply_energy_rules(&mut self, population: &[String]) {
        for ia in population {
            if let Some(energy) = self.energy_levels.get_mut(ia) {
                // Exemple : pénalité pour trop de fichiers
                let file_count = self.count_files(ia);
                if file_count > 10 {
                    *energy -= (file_count as f64 - 10.0) * 0.5; // Pénalité de 0.5 par fichier en excès
                }

                // Exemple : pénalité pour des fichiers trop longs
                let avg_file_length = self.average_file_length(ia);
                if avg_file_length > 500 {
                    *energy -= (avg_file_length - 500.0) * 0.1; // Pénalité de 0.1 par ligne en excès
                }

                // Bonus pour modularité
                if file_count > 1 && avg_file_length < 300 {
                    *energy += 5.0; // Bonus fixe
                }
            }
        }
    }

    /// Retourne le niveau d'énergie d'une IA.
    pub fn get_energy(&self, ia: &str) -> f64 {
        *self.energy_levels.get(ia).unwrap_or(&0.0)
    }

    /// Simule la consommation d'énergie pour une IA.
    fn count_files(&self, ia: &str) -> usize {
        // Simuler le comptage des fichiers (à remplacer par une vraie logique)
        ia.len() % 15 // Exemple arbitraire
    }

    fn average_file_length(&self, ia: &str) -> f64 {
        // Simuler la longueur moyenne des fichiers (à remplacer par une vraie logique)
        ia.len() as f64 * 10.0 // Exemple arbitraire
    }

    /// Déclenche une catastrophe globale qui réduit la population.
    pub fn trigger_global_catastrophe(&self, population: &mut Vec<String>, survival_rate: f64) {
        self.catastrophe_manager
            .trigger_global(population, survival_rate);
    }

    /// Simule le changement climatique.
    pub fn simulate_climate_change(&mut self) {
        self.climate_manager.simulate_climate_change();
    }

    /// Retourne les conditions climatiques actuelles.
    pub fn get_climate_conditions(&self) -> (f64, f64) {
        self.climate_manager.get_conditions()
    }

    /// Simule d'autres facteurs environnementaux (exemple : ressources, climat).
    pub fn simulate_environment(&mut self) {
        // 1. Avancer d'un jour dans le cycle des saisons
        self.season_manager.advance_day();

        // 2. Obtenir les effets saisonniers
        let (temp_effect, humidity_effect) = self.season_manager.get_seasonal_effects();

        // 3. Appliquer les effets saisonniers au climat
        self.climate_manager
            .apply_seasonal_effects(temp_effect, humidity_effect);

        // 4. Simuler le changement climatique
        self.climate_manager.simulate_climate_change();

        // 5. Régénérer les ressources
        self.regenerate_resources();
    }

    /// Analyse le code d'une IA pour déterminer ses besoins en énergie.
    pub fn analyze_code(&self, ia_path: &str) -> f64 {
        let mut energy_needs = 0.0;

        // Lire tous les fichiers de l'IA
        if let Ok(entries) = fs::read_dir(ia_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Calculer les besoins en fonction de la taille du fichier
                        energy_needs += content.len() as f64 * 0.01;

                        // Bonus pour des structures avancées
                        if content.contains("if ") || content.contains("else ") {
                            energy_needs -= 5.0; // Récompense pour l'utilisation de conditions
                        }
                        if content.contains("fn ") {
                            energy_needs -= 2.0; // Récompense pour l'utilisation de fonctions
                        }

                        // Pénalité pour des structures inefficaces
                        if content.contains("unwrap()") {
                            energy_needs += 10.0; // Pénalité pour des pratiques risquées
                        }
                    }
                }
            }
        }

        energy_needs.max(0.0) // Les besoins ne peuvent pas être négatifs
    }

    /// Fournit des ressources aux IA en fonction de l'analyse de leur code.
    pub fn provide_resources_based_on_code(&self, population: &mut Vec<String>) {
        for ia in population {
            let energy_needs = self.analyze_code(ia);
            if let Some(energy) = self.energy_levels.get_mut(ia) {
                *energy += 100.0 - energy_needs; // Fournir des ressources en fonction des besoins
            }
        }
    }

    /// Régénère les ressources de l'environnement.
    pub fn regenerate_resources(&mut self) {
        self.resource_manager.regenerate();

        // Ajuster les ressources en fonction des conditions climatiques
        let (_, _, is_raining) = self.climate_manager.get_conditions();
        if is_raining {
            println!("Il pleut ! Augmentation des ressources en eau.");
            self.resource_manager.consume(-50.0, -100.0, 0.0); // Augmente l'eau disponible
        }
    }

    /// Fournit des ressources aux IA en fonction de leurs besoins et des ressources disponibles.
    pub fn provide_resources(&mut self, population: &mut Vec<String>) {
        for ia in population {
            let mut needs = self.needs_manager.load_needs(ia);

            // Tenter de consommer les ressources nécessaires
            if self
                .resource_manager
                .consume(needs.energy, needs.water, needs.food)
            {
                self.needs_manager.update_needs(
                    &mut needs,
                    &[
                        Resource {
                            name: "energy".to_string(),
                            amount: needs.energy,
                        },
                        Resource {
                            name: "water".to_string(),
                            amount: needs.water,
                        },
                        Resource {
                            name: "food".to_string(),
                            amount: needs.food,
                        },
                    ],
                );
                println!("Ressources fournies à {}: {:?}", ia, needs);
            } else {
                println!("Ressources insuffisantes pour {}", ia);
            }
        }
    }
}
