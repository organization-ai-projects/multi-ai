fn main() {
    let mut explorer = MetaExplorer::new(50);
    let training_data = vec![
        ("salut", "SALUT!"),
        ("hello", "HELLO!"),
        ("test", "TSET!"),
        ("aa", "AA!"),
        ("ab", "AB!"),
        // Ajoute ce que tu veux
    ];

    for gen in 0..100 {
        explorer.evaluate_population(&training_data);
        explorer.natural_selection(10);
        explorer.reproduce(50);

        let best = &explorer.population[0];
        println!(
            "Génération {gen}: best fitness = {:.3}, ops = {:?}",
            best.fitness, best.ops
        );

        if best.fitness > 0.99 {
            println!("Solution trouvée : {:?}", best.ops);
            break;
        }
    }
}
