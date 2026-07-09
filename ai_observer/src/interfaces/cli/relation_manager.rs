use std::io::{self, Write};

use crate::interfaces::cli::utils;

/// Affiche et gère le menu de gestion des relations
pub fn display_menu<T>(
    system: &mut T,
    create_relation_fn: impl Fn(&mut T, &str, &str, &str) -> Result<(), String>,
    add_relation_tag_fn: impl Fn(&mut T, &str, &str, &str) -> Result<(), String>,
    record_activation_result_fn: impl Fn(&mut T, &str, &str, bool) -> Result<(), String>,
    get_all_relations_fn: impl Fn(&mut T) -> Vec<(String, String, String)>, // from, to, relation_type
) {
    utils::show_section("Gestionnaire de Relations");

    loop {
        println!("1. Créer une relation entre concepts");
        println!("2. Ajouter un tag à une relation");
        println!("3. Enregistrer un résultat d'activation");
        println!("4. Visualiser les relations");
        println!("5. Retour au menu principal");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => create_new_relation(system, &create_relation_fn),
            2 => add_tag_to_relation(system, &add_relation_tag_fn),
            3 => record_activation_result(system, &record_activation_result_fn),
            4 => view_all_relations(system, &get_all_relations_fn),
            5 => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

fn create_new_relation<T>(
    system: &mut T,
    create_relation_fn: &impl Fn(&mut T, &str, &str, &str) -> Result<(), String>,
) {
    utils::show_section("Créer une nouvelle relation");

    let from_id = utils::read_input("ID du concept source");
    let to_id = utils::read_input("ID du concept cible");
    let relation_type = utils::read_input("Type de relation");

    match create_relation_fn(system, &from_id, &to_id, &relation_type) {
        Ok(_) => utils::show_success(&format!(
            "Relation '{}' créée avec succès entre '{}' et '{}'",
            relation_type, from_id, to_id
        )),
        Err(e) => utils::show_error(&e),
    }
}

fn add_tag_to_relation<T>(
    system: &mut T,
    add_relation_tag_fn: &impl Fn(&mut T, &str, &str, &str) -> Result<(), String>,
) {
    utils::show_section("Ajouter un tag à une relation");

    let from_id = utils::read_input("ID du concept source");
    let to_id = utils::read_input("ID du concept cible");
    let tag = utils::read_input("Tag à ajouter");

    match add_relation_tag_fn(system, &from_id, &to_id, &tag) {
        Ok(_) => utils::show_success(&format!(
            "Tag '{}' ajouté à la relation entre '{}' et '{}'",
            tag, from_id, to_id
        )),
        Err(e) => utils::show_error(&e),
    }
}

fn record_activation_result<T>(
    system: &mut T,
    record_activation_result_fn: &impl Fn(&mut T, &str, &str, bool) -> Result<(), String>,
) {
    utils::show_section("Enregistrer un résultat d'activation");

    let from_id = utils::read_input("ID du concept source");
    let to_id = utils::read_input("ID du concept cible");
    let success = utils::read_input("Succès? (true/false)");

    let success = match success.as_str() {
        "true" => true,
        "false" => false,
        _ => {
            utils::show_error("Valeur de succès invalide, utilisez 'true' ou 'false'");
            return;
        }
    };

    match record_activation_result_fn(system, &from_id, &to_id, success) {
        Ok(_) => utils::show_success(&format!(
            "Résultat d'activation enregistré pour la relation entre '{}' et '{}'",
            from_id, to_id
        )),
        Err(e) => utils::show_error(&e),
    }
}

fn view_all_relations<T>(
    system: &mut T,
    get_all_relations_fn: &impl Fn(&mut T) -> Vec<(String, String, String)>,
) {
    utils::show_section("Visualiser les relations");

    let relations = get_all_relations_fn(system);

    if relations.is_empty() {
        utils::show_info("Aucune relation trouvée dans le graphe");
    } else {
        for (from_id, to_id, relation_type) in relations {
            println!("Relation de {} à {}: {}", from_id, to_id, relation_type);
        }
    }
}
