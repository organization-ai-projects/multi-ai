/// Orchestrateur pour gérer l'entraînement de l'IA.
pub struct TrainingOrchestrator;

impl TrainingOrchestrator {
    /// Exécute un cycle d'entraînement et retourne les données mises à jour.
    pub fn start_training<T: TrainingData>(data: &mut T) -> T {
        println!("Démarrage de l'entraînement...");
        data.update();
        println!("Données après mise à jour : {}", data.fetch());
        data.clone()
    }
}

/// Trait pour définir les interactions nécessaires avec les données d'entraînement.
pub trait TrainingData {
    fn update(&mut self);
    fn fetch(&self) -> String;
}
