use std::collections::HashMap;
use std::io::{self, Write};

use crate::interfaces::cli::utils;

/// Module pour la création et l'édition des concepts
/// Utilisation d'approche générique pour éviter la dépendance circulaire

/// Affiche et gère le menu d'édition des concepts
pub fn display_menu<T>(
    system: &mut T,
    create_concept_fn: impl Fn(&mut T, &str, &str, HashMap<String, String>) -> Result<(), String>,
    create_concept_with_importance_fn: impl Fn(
        &mut T,
        &str,
        &str,
        HashMap<String, String>,
        f32,
    ) -> Result<(), String>,
    get_all_concepts_fn: impl Fn(&T) -> Vec<(String, String)>,
    tag_conceptual_cluster_fn: impl Fn(&mut T, &[&str], &str) -> Result<(), String>,
    expose_concept_fn: impl Fn(&mut T, &str) -> Result<(), String>,
    set_importance_fn: impl Fn(&mut T, &str, f32) -> Result<(), String>,
    tag_concepts_fn: impl Fn(&mut T, &[&str], &str) -> Result<(), String>,
) {
    utils::show_section("Éditeur de Concepts");

    loop {
        println!("1. Créer un nouveau concept");
        println!("2. Modifier un concept existant");
        println!("3. Définir l'importance d'un concept");
        println!("4. Ajouter des tags aux concepts");
        println!("5. Afficher tous les concepts");
        println!("6. Retour au menu principal");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => create_new_concept(
                system,
                &create_concept_fn,
                &create_concept_with_importance_fn,
            ),
            2 => edit_existing_concept(system, &create_concept_fn),
            3 => {
                let id = utils::read_input("ID du concept");
                let importance =
                    utils::read_number::<f32>("Nouvelle importance (0-1)").unwrap_or(0.5);
                if let Err(e) = set_importance_fn(system, &id, importance) {
                    utils::show_error(&e);
                } else {
                    // Exposer le concept après avoir modifié son importance
                    if let Err(e) = expose_concept_fn(system, &id) {
                        utils::show_error(&e);
                    }
                }
            }
            4 => {
                let ids = utils::read_input("IDs des concepts (séparés par des virgules)");
                let tag = utils::read_input("Tag à ajouter");
                let id_vec: Vec<&str> = ids.split(',').map(str::trim).collect();
                // Utiliser les deux fonctions de tagging pour assurer la compatibilité
                if let Err(e) = tag_conceptual_cluster_fn(system, &id_vec, &tag) {
                    if let Err(e2) = tag_concepts_fn(system, &id_vec, &tag) {
                        utils::show_error(&format!("Erreurs: {}, {}", e, e2));
                    }
                }
            }
            5 => view_all_concepts(system, &get_all_concepts_fn),
            6 => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

/// Crée un nouveau concept dans le système
fn create_new_concept<T>(
    system: &mut T,
    create_concept_fn: &impl Fn(&mut T, &str, &str, HashMap<String, String>) -> Result<(), String>,
    create_concept_with_importance_fn: &impl Fn(
        &mut T,
        &str,
        &str,
        HashMap<String, String>,
        f32,
    ) -> Result<(), String>,
) {
    utils::show_section("Création d'un nouveau concept");

    let id = utils::read_input("ID du concept");
    let content = utils::read_input("Contenu du concept");
    let importance_str = utils::read_input("Importance (1-10, ou vide pour défaut)");

    let mut metadata = HashMap::new();
    metadata.insert("créateur".to_string(), "utilisateur".to_string());
    metadata.insert(
        "date_de_création".to_string(),
        chrono::Utc::now().to_string(),
    );

    if let Some(category) = utils::read_input_optional("Catégorie (optionnel)") {
        metadata.insert("catégorie".to_string(), category);
    }

    if importance_str.is_empty() {
        match create_concept_fn(system, &id, &content, metadata) {
            Ok(_) => utils::show_success(&format!("Concept '{}' créé avec succès", id)),
            Err(e) => utils::show_error(&format!("Erreur: {}", e)),
        }
    } else {
        match importance_str.parse::<f32>() {
            Ok(importance) => {
                let normalized_importance = importance / 10.0;
                match create_concept_with_importance_fn(
                    system,
                    &id,
                    &content,
                    metadata,
                    normalized_importance,
                ) {
                    Ok(_) => utils::show_success(&format!(
                        "Concept '{}' créé avec une importance de {:.2}",
                        id, normalized_importance
                    )),
                    Err(e) => utils::show_error(&format!("Erreur: {}", e)),
                }
            }
            Err(_) => {
                utils::show_error("Valeur d'importance invalide");
                match create_concept_fn(system, &id, &content, metadata) {
                    Ok(_) => utils::show_success(&format!(
                        "Concept '{}' créé avec importance par défaut",
                        id
                    )),
                    Err(e) => utils::show_error(&format!("Erreur: {}", e)),
                }
            }
        }
    }
}

/// Modifie un concept existant dans le système
fn edit_existing_concept<T>(
    system: &mut T,
    create_concept_fn: &impl Fn(&mut T, &str, &str, HashMap<String, String>) -> Result<(), String>,
) {
    utils::show_section("Modification d'un concept existant");

    let id = utils::read_input("ID du concept à modifier");
    let new_content = utils::read_input("Nouveau contenu du concept");

    let mut metadata = HashMap::new();
    metadata.insert("modificateur".to_string(), "utilisateur".to_string());
    metadata.insert(
        "date_modification".to_string(),
        chrono::Utc::now().to_string(),
    );

    match create_concept_fn(system, &id, &new_content, metadata) {
        Ok(_) => utils::show_success(&format!("Concept '{}' modifié avec succès", id)),
        Err(e) => utils::show_error(&format!("Erreur: {}", e)),
    }
}

/// Affiche tous les concepts présents dans le système
fn view_all_concepts<T>(system: &T, get_all_concepts_fn: &impl Fn(&T) -> Vec<(String, String)>) {
    utils::show_section("Liste de tous les concepts");

    let concepts = get_all_concepts_fn(system);

    if concepts.is_empty() {
        utils::show_info("Aucun concept trouvé dans le système.");
        return;
    }

    for (id, content) in concepts {
        println!("- {}: {}", id, content);
    }
}
