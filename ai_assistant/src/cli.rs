use clap::{App, Arg, SubCommand};
use crate::graph_memory::AiGraph;

pub fn parse_and_execute() {
    let graph = AiGraph::new(); // Enlever mut car non utilisé

    let matches = App::new("AI Assistant")
        .version("0.1.0")
        .author("Votre Nom <votre.email@example.com>")
        .about("Assistant IA pour la gestion des versions")
        .subcommand(
            SubCommand::with_name("suggest")
                .about("Analyse un projet et propose des actions")
                .arg(Arg::with_name("path").required(true).help("Chemin du projet à analyser")),
        )
        .subcommand(
            SubCommand::with_name("log")
                .about("Affiche le journal des décisions"),
        )
        .subcommand(
            SubCommand::with_name("memory")
                .about("Gère la mémoire de l'IA")
                .subcommand(SubCommand::with_name("show").about("Affiche la mémoire actuelle")),
        )
        .subcommand(
            SubCommand::with_name("decision")
                .about("Explique une décision spécifique")
                .arg(Arg::with_name("id").required(true).help("ID de la décision")),
        )
        .subcommand(
            SubCommand::with_name("feedback")
                .about("Ajoute un feedback utilisateur à une décision")
                .arg(Arg::with_name("id").required(true).help("ID de la décision"))
                .arg(Arg::with_name("feedback").required(true).help("Feedback (good/bad)")),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("suggest") {
        let path = matches.value_of("path").unwrap();
        println!("Analyse du projet à : {}", path);
        // Appeler la logique d'analyse ici
    } else if matches.subcommand_matches("log").is_some() {
        println!("Affichage du journal des décisions...");
        // Appeler la logique pour afficher le journal
    } else if let Some(matches) = matches.subcommand_matches("memory") {
        if matches.subcommand_matches("show").is_some() {
            println!("État du graphe : {:?}", graph);
        }
    } else if let Some(matches) = matches.subcommand_matches("decision") {
        let id = matches.value_of("id").unwrap();
        println!("Explication de la décision ID : {}", id);
        // Appeler la logique pour expliquer une décision
    } else if let Some(matches) = matches.subcommand_matches("feedback") {
        let id = matches.value_of("id").unwrap();
        let feedback = matches.value_of("feedback").unwrap();
        println!("Feedback '{}' ajouté pour la décision ID : {}", feedback, id);
        // Appeler la logique pour enregistrer le feedback
    }
}
