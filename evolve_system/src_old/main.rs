/// Ce fichier est le point d'entrée principal de l'application.
/// Rôle : Permettre à l'IA de s'auto-gérer et d'expérimenter librement via `Experiment`.
mod experiment;
mod explorer;
mod fitness;
mod strategies; // Ajout de la déclaration du module `fitness`

use experiment::Experiment;

fn main() {
    // 1. Initialisation de l'expérimentation avec un moteur de fitness par défaut
    let mut experiment = Experiment::new_with_default_fitness("42");

    // 2. Ajouter des opérations dynamiques
    experiment.add_custom_op("square", Box::new(|x| x * x));
    experiment.add_custom_op("cube", Box::new(|x| x * x * x));

    // 3. Ajouter des stratégies
    experiment.add_strategy_from_pipeline(vec!["square", "add_5"]);
    experiment.add_strategy_from_pipeline(vec!["cube", "add_10"]);

    // 4. Configurer les transformations pour l'explorateur
    let transformations: Vec<Box<dyn Fn(&str) -> String + Send + Sync>> = vec![
        Box::new(|input| format!("{}!", input)),
        Box::new(|input| input.chars().rev().collect()),
    ];

    // 5. Exécuter une expérimentation complète
    let (strategy_results, exploration_results, random_result) = experiment.run(&transformations);

    // 6. Afficher les résultats
    println!("Résultats des stratégies : {:?}", strategy_results);
    println!("Résultats des explorations : {:?}", exploration_results);
    println!(
        "Résultat de la transformation aléatoire : {:?}",
        random_result
    );

    // 7. Appliquer une transformation composée
    if let Some(meta_result) = experiment.run_meta_explore(&transformations) {
        println!("Résultat de la transformation composée : {:?}", meta_result);
    } else {
        println!("Impossible de créer une transformation composée.");
    }
}
