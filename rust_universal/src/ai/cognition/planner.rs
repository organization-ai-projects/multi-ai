use crate::ai::brain::BrainCognition;

pub fn reason(cognition: &mut BrainCognition) {
    cognition.associate(1, 2, 0.9);

    if cognition.validate() {
        println!("🧠 Mémoire valide !");
    }
}

pub fn find_path(cognition: &BrainCognition, start: usize, end: usize) -> Option<Vec<usize>> {
    if cognition.validate() { // Utilise la méthode publique validate() au lieu d'accéder à memory
        Some(vec![start, end])
    } else {
        None
    }
}

pub fn generate_plan(cognition: &BrainCognition) -> Vec<String> {
    vec![
        "Analyser les données".to_string(),
        "Prendre une décision".to_string(),
        "Exécuter l'action".to_string(),
    ]
}

pub fn run_cognition(cognition: &mut BrainCognition) {
    reason(cognition);
}
