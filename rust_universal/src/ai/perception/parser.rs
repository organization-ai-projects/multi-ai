use crate::ai::brain::BrainPerception;

/// Prétraite l'entrée utilisateur et alimente la mémoire via la perception
pub fn run_perception(perception: &mut BrainPerception, input: &str) {
    let input = input.trim();
    if input.is_empty() {
        println!("⚠️ Entrée vide ignorée.");
        return;
    }

    perception.observe_text(input);
    let keywords = extract_keywords(input);
    println!("🔍 Mots-clés extraits : {:?}", keywords);
}

/// Extraction naïve de mots-clés à partir d'une chaîne
fn extract_keywords(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .filter(|word| word.len() > 3)
        .map(|word| word.to_lowercase())
        .collect()
}
