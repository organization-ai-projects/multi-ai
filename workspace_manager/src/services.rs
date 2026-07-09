use std::collections::HashMap;
use std::process::{Child, Command};

pub struct ServiceManager {
    services: HashMap<String, Option<Child>>,
    dependencies: HashMap<String, Vec<String>>, // Dépendances entre services
}

impl ServiceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            services: HashMap::new(),
            dependencies: HashMap::new(),
        };

        // Définir les dépendances
        manager.dependencies.insert(
            "ide_web".to_string(),
            vec!["ai_assistant".to_string(), "version_watcher".to_string()],
        );
        manager.dependencies.insert(
            "ai_assistant".to_string(),
            vec!["semver_planner".to_string()],
        );
        manager.dependencies.insert(
            "version_watcher".to_string(),
            vec!["semver_planner".to_string()],
        );

        // Initialiser les services disponibles avec leur état (tous arrêtés au départ)
        manager.services.insert("ide_web".to_string(), None);
        manager.services.insert("ai_assistant".to_string(), None);
        manager.services.insert("version_watcher".to_string(), None);
        manager.services.insert("semver_planner".to_string(), None);

        manager
    }

    /// Démarre un service (ex: ide_web, ai_assistant, version_watcher, semver_planner)
    pub fn start_service(&mut self, name: &str, command: &str, args: &[&str]) -> bool {
        if self.services.contains_key(name) {
            eprintln!("Service '{}' est déjà démarré.", name);
            return false;
        }

        match Command::new(command).args(args).spawn() {
            Ok(child) => {
                self.services.insert(name.to_string(), Some(child));
                println!("Service '{}' démarré.", name);

                // Démarrer les dépendances si nécessaire
                if let Some(dependencies) = self.dependencies.get(name).cloned() {
                    // Clonez ici
                    for dependency in dependencies {
                        if !self.services.contains_key(&dependency) {
                            self.start_service(
                                &dependency,
                                "cargo",
                                &[
                                    "run",
                                    "--manifest-path",
                                    &format!("../{}/Cargo.toml", dependency),
                                ],
                            );
                        }
                    }
                }

                true
            }
            Err(e) => {
                eprintln!("Erreur lors du démarrage du service '{}': {}", name, e);
                false
            }
        }
    }

    /// Arrête un service
    pub fn stop_service(&mut self, name: &str) -> bool {
        if let Some(Some(child)) = self.services.get_mut(name) {
            if let Err(e) = child.kill() {
                eprintln!("Erreur lors de l'arrêt du service '{}': {}", name, e);
                return false;
            }
            self.services.insert(name.to_string(), None);
            println!("Service '{}' arrêté.", name);

            // Arrêter les services dépendants
            for (dependent, dependencies) in self.dependencies.clone() {
                // Clonez ici
                if dependencies.contains(&name.to_string()) {
                    self.stop_service(&dependent);
                }
            }

            true
        } else {
            eprintln!("Service '{}' n'est pas en cours d'exécution.", name);
            false
        }
    }

    /// Liste tous les services et leur état
    pub fn list_services(&self) -> Vec<serde_json::Value> {
        self.services
            .iter()
            .map(|(name, child)| {
                serde_json::json!({
                    "name": name,
                    "running": child.is_some()
                })
            })
            .collect()
    }
}
