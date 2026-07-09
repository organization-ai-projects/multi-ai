use crate::ai::brain::BrainDialogue;

pub fn process_dialogue(dialogue: &mut BrainDialogue, input: &str) {
    // Stocke le dialogue dans la mémoire et récupère son ID
    let dialogue_id = dialogue.store_dialogue(input);

    // Recherche une réponse dynamique dans la mémoire
    if let Some(response) = dialogue.retrieve_response(input) {
        println!("{}", response);
        let response_id = dialogue.store_dialogue(&response);
        dialogue.link_dialogues_bidirectional(dialogue_id, response_id, 0.9);
    } else {
        // Si aucune réponse n'est trouvée, crée une réponse par défaut
        let default_response = "Je ne comprends pas.";
        println!("{}", default_response);
        let response_id = dialogue.store_dialogue(default_response);
        dialogue.link_dialogues_bidirectional(dialogue_id, response_id, 0.5);
    }
}

pub fn link_related_dialogues(dialogues: &[(usize, usize, f32)]) {
    let mut dialogue = BrainDialogue::new();

    for &(from, to, relevance) in dialogues {
        println!(
            "🔗 Création d'un lien bidirectionnel entre dialogues : {} ↔ {}",
            from, to
        );
        dialogue.link_dialogues_bidirectional(from, to, relevance);
    }
}
