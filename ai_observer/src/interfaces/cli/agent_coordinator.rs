use std::collections::HashMap;
use std::io::{self, Write};

use crate::interfaces::cli::utils;

/// Affiche et gère le menu de coordination entre agents IA
pub fn display_menu<T>(
    system: &mut T,
    create_agent_fn: impl Fn(&mut T, &str) -> Result<(), String>,
    create_concept_in_agent_fn: impl Fn(
        &mut T,
        &str,
        &str,
        &str,
        HashMap<String, String>,
    ) -> Result<(), String>,
    share_concept_fn: impl Fn(&mut T, &str, &str) -> Result<(), String>,
    broadcast_concept_fn: impl Fn(&mut T, &str) -> HashMap<String, Result<(), String>>,
    get_sharing_stats_fn: impl Fn(&T) -> HashMap<String, usize>,
) {
    utils::show_section("Coordinateur d'Agents IA");

    loop {
        println!("1. Créer un nouvel agent IA");
        println!("2. Créer un concept dans un agent");
        println!("3. Partager un concept entre agents");
        println!("4. Diffuser un concept à tous les agents");
        println!("5. Analyser les contributions des agents");
        println!("6. Retour au menu principal");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => create_new_agent(system, &create_agent_fn),
            2 => create_concept_in_agent(system, &create_concept_in_agent_fn),
            3 => share_concept_between_agents(system, &share_concept_fn),
            4 => broadcast_concept_to_all(system, &broadcast_concept_fn),
            5 => analyze_agent_contributions(system, &get_sharing_stats_fn),
            6 => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

fn create_new_agent<T>(
    system: &mut T,
    create_agent_fn: &impl Fn(&mut T, &str) -> Result<(), String>,
) {
    utils::show_section("Créer un nouvel agent IA");

    let ai_id = utils::read_input("Identifiant de l'agent");

    match create_agent_fn(system, &ai_id) {
        Ok(_) => utils::show_success(&format!("Agent IA '{}' créé avec succès", ai_id)),
        Err(e) => utils::show_error(&e),
    }
}

fn create_concept_in_agent<T>(
    system: &mut T,
    create_concept_in_agent_fn: &impl Fn(
        &mut T,
        &str,
        &str,
        &str,
        HashMap<String, String>,
    ) -> Result<(), String>,
) {
    utils::show_section("Créer un concept dans un agent");

    let ai_id = utils::read_input("Identifiant de l'agent");
    let concept_id = utils::read_input("Identifiant du concept");
    let content = utils::read_input("Contenu du concept");

    let mut metadata = HashMap::new();
    metadata.insert("agent_créateur".to_string(), ai_id.clone());
    metadata.insert("date_création".to_string(), chrono::Utc::now().to_string());

    match create_concept_in_agent_fn(system, &ai_id, &concept_id, &content, metadata) {
        Ok(_) => utils::show_success(&format!(
            "Concept '{}' créé avec succès dans l'agent '{}'",
            concept_id, ai_id
        )),
        Err(e) => utils::show_error(&e),
    }
}

fn share_concept_between_agents<T>(
    system: &mut T,
    share_concept_fn: &impl Fn(&mut T, &str, &str) -> Result<(), String>,
) {
    utils::show_section("Partager un concept entre agents");

    let source_ai_id = utils::read_input("Identifiant de l'agent source");
    let concept_id = utils::read_input("Identifiant du concept à partager");

    match share_concept_fn(system, &source_ai_id, &concept_id) {
        Ok(_) => utils::show_success(&format!(
            "Concept '{}' partagé avec succès depuis l'agent '{}'",
            concept_id, source_ai_id
        )),
        Err(e) => utils::show_error(&e),
    }
}

fn broadcast_concept_to_all<T>(
    system: &mut T,
    broadcast_concept_fn: &impl Fn(&mut T, &str) -> HashMap<String, Result<(), String>>,
) {
    utils::show_section("Diffuser un concept à tous les agents");

    let concept_id = utils::read_input("Identifiant du concept à diffuser");

    let results = broadcast_concept_fn(system, &concept_id);

    println!("Résultats de la diffusion:");
    for (ai_id, result) in results {
        match result {
            Ok(_) => println!("- {}: succès", ai_id),
            Err(e) => println!("- {}: échec ({})", ai_id, e),
        }
    }
}

fn analyze_agent_contributions<T>(
    system: &T,
    get_sharing_stats_fn: &impl Fn(&T) -> HashMap<String, usize>,
) {
    utils::show_section("Analyser les contributions des agents");

    let stats = get_sharing_stats_fn(system);

    if stats.is_empty() {
        utils::show_info("Aucune contribution n'a encore été faite.");
    } else {
        println!("Contributions par agent:");
        for (ai_id, count) in stats {
            println!("- {}: {} concept(s)", ai_id, count);
        }
    }
}
