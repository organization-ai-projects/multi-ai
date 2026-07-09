fn main() {
    // Essaie de charger la mémoire précédente
    let mut ai = LearningLoop::load("mem").unwrap_or_else(LearningLoop::new);

    println!("Tape un mot, une phrase, ce que tu veux (Ctrl+C pour quitter)");
    loop {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let input = buf.trim();
        if input.is_empty() {
            continue;
        }
        ai.learn(input);

        // Optionnel : affichage simplifié
        let voisins = &ai.graph.nodes[input].neighbors;
        println!("Liens de {input} : {:?}", voisins);

        // Persiste l’état à chaque itération (sécurité 💾)
        ai.save("mem");
    }
}
