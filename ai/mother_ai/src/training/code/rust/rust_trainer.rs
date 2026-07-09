use crate::training::training_types::Trainer; // Mise à jour pour référencer `Trainer` depuis `training_types.rs`

#[derive(Clone, Debug)] // Ajout de Debug
pub struct RustTrainer;

impl Trainer for RustTrainer {
    fn train(&self, data: &str) {
        println!("Entraînement sur du code Rust avec les données : {}", data);
        // Logique spécifique à l'entraînement Rust
    }
}
