use crate::training::training_types::TrainingKind;
use crate::training::trainer_loader::load_trainer;
use rand::seq::SliceRandom; // Utilisé pour la méthode .choose()

/// Exécute un entraînement aléatoire.
pub fn execute_training(data: &str) -> Result<String, String> {
    let kinds = vec![
        TrainingKind::Rust,
        TrainingKind::PHP,
        TrainingKind::NLP,
        TrainingKind::Audio,
    ];

    let mut rng = rand::rng();
    let selected = kinds
        .choose(&mut rng)
        .ok_or("Aucune stratégie disponible")?;

    println!("🎯 Entraînement sélectionné : {:?}", selected);
    let trainer = load_trainer(selected);
    trainer.train(data);

    // Retourne le type d'entraînement en tant que chaîne
    Ok(format!("{:?}", selected))
}
