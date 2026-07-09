fn main() {
    let mut explorer = Explorer::new();

    let inputs = vec!["salut", "coucou", "42", "ai"];

    // Exploration auto de la population
    explorer.explore_population(&inputs);

    // Affiche les “meilleures” expériences
    for exp in explorer.best_results() {
        println!(
            "Input: {:?} => Result: {:?} | Succès: {}",
            exp.input, exp.result, exp.success
        );
    }

    // Tu peux aussi tester du random sur de nouveaux inputs
    let input = "hello";
    let exp = explorer.try_random_action(input);
    println!(
        "Random: {:?} => {:?} | Succès: {}",
        exp.input, exp.result, exp.success
    );
}
