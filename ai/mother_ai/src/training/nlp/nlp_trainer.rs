use crate::training::training_types::Trainer; // Mise à jour pour référencer `Trainer` depuis `training_types.rs`

#[derive(Clone, Debug)] // Ajout de Debug
pub struct NlpTrainer;

impl Trainer for NlpTrainer {
    fn train(&self, data: &str) {
        println!("Entraînement NLP avec les données : {}", data);
        // Logique spécifique à l'entraînement NLP
    }
}
