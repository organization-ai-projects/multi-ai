use crate::ai::brain::BrainLearning;

pub fn train(concept_id: usize, success: bool) {
    let mut learning = BrainLearning::new();
    println!("🔁 Entraînement sur concept {concept_id}");
    if success {
        learning.reinforce_success(concept_id);
        learning.mark_concept_as_successful(concept_id);
    } else {
        println!("❌ Concept {concept_id} marqué comme échoué.");
        learning.weaken_failure(concept_id);
        learning.mark_concept_as_failed(concept_id);
    }
}

pub fn supervised_train(pairs: &[(usize, usize, f32, bool)]) {
    let mut learning = BrainLearning::new();

    for &(from, to, weight, success) in pairs {
        println!("🔗 Association supervisée : {from} → {to} (poids: {weight})");
        learning.associate_bidirectional(from, to, weight);
        if success {
            learning.reinforce_success(from);
            learning.mark_concept_as_successful(from);
        } else {
            println!("❌ Concept {from} marqué comme échoué.");
            learning.weaken_failure(from);
            learning.mark_concept_as_failed(from);
        }
    }

    println!("✅ Apprentissage supervisé terminé");
}

pub fn extract_strategies_from_training() {
    println!("🧠 Extraction des stratégies des sessions d'entraînement...");
    let mut learning = BrainLearning::new();

    // Récupère les concepts et leurs résultats depuis la mémoire
    let concepts = vec![1, 2, 3, 4, 5]; // Liste des concepts à analyser
    for concept_id in concepts {
        if let Some(success) = learning.was_concept_successful(concept_id) {
            if success {
                println!("✅ Concept {concept_id} renforcé avec succès.");
                learning.reinforce_success(concept_id);
            } else {
                println!("❌ Concept {concept_id} affaibli (échec).");
                learning.weaken_failure(concept_id);
            }
        } else {
            println!("⚠️ Aucun résultat trouvé pour le concept {concept_id}.");
        }
    }
}

pub fn adjust_association_weights(pairs: &[(usize, usize, f32)]) {
    let mut learning = BrainLearning::new();

    for &(from, to, delta) in pairs {
        println!("⚖️ Ajustement du poids du lien : {from} → {to} (delta: {delta})");
        learning.adjust_association_weight(from, to, delta);
    }
}
