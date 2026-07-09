pub fn evaluate_goals() {
    // Implémentez la logique pour évaluer les objectifs ici
    println!("Évaluation des objectifs...");
}

pub fn prioritize_goals(goals: &mut Vec<(usize, f32)>) {
    goals.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    println!("🎯 Objectifs priorisés : {:?}", goals);
}
