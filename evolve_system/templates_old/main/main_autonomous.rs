use std::time::Duration;

fn main() {
    let mut population = Population::new(50);
    let mut fitness_engine = FitnessEngine::new();
    let mut gen = 0;

    loop {
        // Possibilité de muter la fitness elle-même
        if gen % 20 == 0 {
            fitness_engine.mutate_rule();
            println!("Fitness évoluée !");
        }

        // Évaluation
        population.evaluate(&|s| fitness_engine.evaluate(s));

        // Logging
        let best = population
            .individuals
            .iter()
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
            .unwrap();
        println!(
            "Génération {}: meilleur fitness = {:.4}, pipeline = {:?}",
            gen, best.fitness, best.pipeline
        );

        // Sélection / crossover / mutation
        population.next_generation(0.5, 0.6);

        // Laisser l'IA explorer, rater, tenter tout, faire muter ses pipelines, ses règles de fitness, etc.
        std::thread::sleep(Duration::from_millis(500));
        gen += 1;
    }
}
