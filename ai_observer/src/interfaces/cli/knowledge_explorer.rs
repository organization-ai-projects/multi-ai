use crate::interfaces::cli::utils;
use std::collections::HashMap;
use std::io::{self, Write};

// Tous les paramètres sont passés de façon explicite pour éviter les dépendances directes
pub fn display_menu<T>(
    system: &mut T,
    find_related_concepts: impl Fn(&T, &str, usize) -> Result<Vec<(String, f32)>, String>,
    find_path: impl Fn(&T, &str, &str) -> Result<Vec<String>, String>,
    find_concepts_by_tag: impl Fn(&T, &str) -> Vec<String>,
    find_relations_by_tag: impl Fn(&T, &str) -> Vec<(String, String)>,
    get_system_stats: impl Fn(&T) -> HashMap<String, String>,
    get_concept_weight: impl Fn(&T, &str) -> Result<f32, String>,
) {
    utils::show_section("Explorateur de Connaissances");

    loop {
        println!("1. Rechercher des concepts associés");
        println!("2. Trouver un chemin entre concepts");
        println!("3. Rechercher par tag conceptuel");
        println!("4. Rechercher des relations par tag");
        println!("5. Voir les statistiques du système");
        println!("6. Voir les détails d'un concept");
        println!("7. Retour au menu principal");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => find_related_concepts_menu(system, &find_related_concepts),
            2 => find_path_between_concepts_menu(system, &find_path),
            3 => search_by_concept_tag_menu(system, &find_concepts_by_tag),
            4 => search_relations_by_tag_menu(system, &find_relations_by_tag),
            5 => view_system_statistics_menu(system, &get_system_stats),
            6 => {
                let concept_id = utils::read_input("ID du concept");
                view_concept_details(system, &get_concept_weight, &concept_id);
            }
            7 => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

fn find_related_concepts_menu<T>(
    cli: &mut T,
    find_related_fn: impl Fn(&T, &str, usize) -> Result<Vec<(String, f32)>, String>,
) {
    utils::show_section("Recherche de Concepts Associés");

    let concept_id = utils::read_input("ID du concept");
    let limit = utils::read_number::<usize>("Nombre de résultats (défaut: 10)").unwrap_or(10);

    match find_related_fn(cli, &concept_id, limit) {
        Ok(related_nodes) => {
            if related_nodes.is_empty() {
                utils::show_info("Aucun concept associé trouvé.");
            } else {
                println!("Concepts associés à '{}':", concept_id);
                for (i, (id, score)) in related_nodes.iter().enumerate() {
                    println!("{}. {} (score: {:.2})", i + 1, id, score);
                }
            }
        }
        Err(e) => utils::show_error(&e),
    }
}

fn find_path_between_concepts_menu<T>(
    cli: &mut T,
    find_path_fn: impl Fn(&T, &str, &str) -> Result<Vec<String>, String>,
) {
    utils::show_section("Recherche de Chemin entre Concepts");

    let from_id = utils::read_input("ID du concept de départ");
    let to_id = utils::read_input("ID du concept d'arrivée");

    match find_path_fn(cli, &from_id, &to_id) {
        Ok(path) => {
            println!("Chemin trouvé entre '{}' et '{}':", from_id, to_id);
            for (i, step) in path.iter().enumerate() {
                println!("{}. {}", i + 1, step);
            }
        }
        Err(e) => utils::show_error(&e),
    }
}

fn search_by_concept_tag_menu<T>(
    cli: &mut T,
    find_concepts_by_tag_fn: impl Fn(&T, &str) -> Vec<String>,
) {
    utils::show_section("Recherche par Tag Conceptuel");

    let tag = utils::read_input("Tag à rechercher");
    let results = find_concepts_by_tag_fn(cli, &tag);

    utils::show_numbered_list(&results, "Aucun concept trouvé avec ce tag.");
}

fn search_relations_by_tag_menu<T>(
    cli: &mut T,
    find_relations_by_tag_fn: impl Fn(&T, &str) -> Vec<(String, String)>,
) {
    utils::show_section("Recherche de Relations par Tag");

    let tag = utils::read_input("Tag de relation à rechercher");
    let results: Vec<String> = find_relations_by_tag_fn(cli, &tag)
        .iter()
        .map(|(from, to)| format!("{} <-> {}", from, to))
        .collect();

    utils::show_numbered_list(&results, "Aucune relation trouvée avec ce tag.");
}

fn view_system_statistics_menu<T>(
    cli: &mut T,
    get_system_stats_fn: impl Fn(&T) -> HashMap<String, String>,
) {
    utils::show_section("Statistiques du Système");

    let stats = get_system_stats_fn(cli);

    println!("Statistiques du Graphe de Connaissances:");
    for (key, value) in stats {
        println!("{}: {}", key, value);
    }
}

fn view_concept_details<T>(
    system: &T,
    get_concept_weight_fn: &impl Fn(&T, &str) -> Result<f32, String>,
    concept_id: &str,
) {
    utils::show_section("Détails du Concept");

    match get_concept_weight_fn(system, concept_id) {
        Ok(weight) => println!("Poids actuel du concept: {:.3}", weight),
        Err(e) => utils::show_error(&e),
    }
}
