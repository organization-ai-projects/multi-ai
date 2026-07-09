use crate::training::training_types::Trainer; // Mise à jour pour référencer `Trainer` depuis `training_types.rs`

#[derive(Clone, Debug)] // Ajout de Debug
pub struct PhpTrainer;

impl Trainer for PhpTrainer {
    fn train(&self, data: &str) {
        println!("Entraînement sur du code PHP avec les données : {}", data);
        // Logique spécifique à l'entraînement PHP
    }
}
