use crate::training::training_types::Trainer; // Mise à jour pour référencer `Trainer` depuis `training_types.rs`

#[derive(Clone, Debug)] // Ajout de Debug
pub struct AudioTrainer;

impl Trainer for AudioTrainer {
    fn train(&self, data: &str) {
        println!("Entraînement sur des données audio avec : {}", data);
        // Logique spécifique à l'entraînement audio
    }
}
