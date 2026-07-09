use std::io::{self, Write};

use crate::entities::memory::memory_types::{ArchivalStatus, MemoryType};
use crate::interfaces::cli::utils;

/// Affiche et gère le menu de gestion de la mémoire
pub fn display_menu<T>(
    system: &mut T,
    expose_concept_fn: impl Fn(&mut T, &str) -> Result<(), String>,
    archive_concept_fn: impl Fn(&mut T, &str) -> Result<(), String>,
    unarchive_concept_fn: impl Fn(&mut T, &str) -> Result<(), String>,
    promote_concept_memory_fn: impl Fn(&mut T, &str, &str) -> Result<bool, String>,
    get_memory_statistics_fn: impl Fn(&T) -> String,
    find_concepts_by_memory_type_fn: impl Fn(&T, MemoryType) -> Vec<String>,
    find_concepts_by_archival_status_fn: impl Fn(&T, ArchivalStatus) -> Vec<String>,
    get_concept_memory_transitions_fn: impl Fn(
        &T,
        &str,
    )
        -> Result<Vec<(String, String, String, String)>, String>,
    create_memory_snapshot_fn: impl Fn(&T, &str, &str) -> Result<String, String>,
    list_memory_snapshots_fn: impl Fn(&T, &str) -> Result<Vec<String>, String>,
    load_memory_snapshot_fn: impl Fn(&mut T, &str) -> Result<(), String>,
) {
    utils::show_section("Gestionnaire de Mémoire");

    loop {
        println!("1. Exposer un concept");
        println!("2. Archiver/désarchiver un concept");
        println!("3. Promouvoir un concept");
        println!("4. Voir les statistiques de mémoire");
        println!("5. Rechercher par type de mémoire");
        println!("6. Voir l'historique des transitions");
        println!("7. Créer un snapshot de mémoire");
        println!("8. Charger un snapshot");
        println!("9. Retour au menu principal");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => expose_concept(system, &expose_concept_fn),
            2 => toggle_archive_concept(system, &archive_concept_fn, &unarchive_concept_fn),
            3 => promote_concept(system, &promote_concept_memory_fn),
            4 => view_memory_statistics(system, &get_memory_statistics_fn),
            5 => search_by_memory_type(
                system,
                &find_concepts_by_memory_type_fn,
                &find_concepts_by_archival_status_fn,
            ),
            6 => view_memory_transitions(system, &get_concept_memory_transitions_fn),
            7 => create_memory_snapshot(system, &create_memory_snapshot_fn),
            8 => load_memory_snapshot(system, &list_memory_snapshots_fn, &load_memory_snapshot_fn),
            9 => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

// Fonctions pour chaque opération du menu, adaptées pour utiliser les fonctions de rappel
fn expose_concept<T>(
    system: &mut T,
    expose_concept_fn: &impl Fn(&mut T, &str) -> Result<(), String>,
) {
    utils::show_section("Exposer un Concept");

    let id = utils::read_input("ID du concept à exposer");

    match expose_concept_fn(system, &id) {
        Ok(_) => utils::show_success(&format!("Concept '{}' exposé avec succès", id)),
        Err(e) => utils::show_error(&e),
    }
}

fn toggle_archive_concept<T>(
    system: &mut T,
    archive_concept_fn: &impl Fn(&mut T, &str) -> Result<(), String>,
    unarchive_concept_fn: &impl Fn(&mut T, &str) -> Result<(), String>,
) {
    utils::show_section("Archiver/Désarchiver un Concept");

    let id = utils::read_input("ID du concept");

    println!("1. Archiver le concept");
    println!("2. Désarchiver le concept");
    print!("Votre choix: ");
    io::stdout().flush().unwrap();

    match utils::read_menu_choice() {
        1 => match archive_concept_fn(system, &id) {
            Ok(_) => utils::show_success(&format!("Concept '{}' archivé avec succès", id)),
            Err(e) => utils::show_error(&e),
        },
        2 => match unarchive_concept_fn(system, &id) {
            Ok(_) => utils::show_success(&format!("Concept '{}' désarchivé avec succès", id)),
            Err(e) => utils::show_error(&e),
        },
        _ => println!("Option invalide"),
    }
}

fn promote_concept<T>(
    system: &mut T,
    promote_concept_memory_fn: &impl Fn(&mut T, &str, &str) -> Result<bool, String>,
) {
    utils::show_section("Promouvoir un Concept");

    let id = utils::read_input("ID du concept à promouvoir");
    let cause = utils::read_input("Cause de la promotion");

    match promote_concept_memory_fn(system, &id, &cause) {
        Ok(true) => utils::show_success(&format!("Concept '{}' promu avec succès", id)),
        Ok(false) => utils::show_info("Le concept est déjà au niveau maximal (LTM)"),
        Err(e) => utils::show_error(&e),
    }
}

fn view_memory_statistics<T>(system: &T, get_memory_statistics_fn: &impl Fn(&T) -> String) {
    utils::show_section("Statistiques de Mémoire");
    // Utiliser directement la String retournée sans appeler format()
    println!("{}", get_memory_statistics_fn(system));
}

fn search_by_memory_type<T>(
    system: &mut T,
    find_concepts_by_memory_type_fn: &impl Fn(&T, MemoryType) -> Vec<String>,
    find_concepts_by_archival_status_fn: &impl Fn(&T, ArchivalStatus) -> Vec<String>,
) {
    utils::show_section("Recherche par Type de Mémoire");

    println!("Sélectionner le type de mémoire:");
    println!("1. STM (court terme)");
    println!("2. MTM (moyen terme)");
    println!("3. LTM (long terme)");
    println!("4. Concepts archivés");
    println!("5. Concepts actifs");
    print!("Votre choix: ");
    io::stdout().flush().unwrap();

    match utils::read_menu_choice() {
        1 => display_concepts_by_type(
            system,
            find_concepts_by_memory_type_fn,
            MemoryType::STM,
            "Concepts en mémoire à court terme",
        ),
        2 => display_concepts_by_type(
            system,
            find_concepts_by_memory_type_fn,
            MemoryType::MTM,
            "Concepts en mémoire à moyen terme",
        ),
        3 => display_concepts_by_type(
            system,
            find_concepts_by_memory_type_fn,
            MemoryType::LTM,
            "Concepts en mémoire à long terme",
        ),
        4 => display_concepts_by_status(
            system,
            find_concepts_by_archival_status_fn,
            ArchivalStatus::Archived,
            "Concepts archivés",
        ),
        5 => display_concepts_by_status(
            system,
            find_concepts_by_archival_status_fn,
            ArchivalStatus::Active,
            "Concepts actifs",
        ),
        _ => println!("Option invalide"),
    }
}

fn display_concepts_by_type<T>(
    system: &mut T,
    find_concepts_by_memory_type_fn: &impl Fn(&T, MemoryType) -> Vec<String>,
    memory_type: MemoryType,
    title: &str,
) {
    utils::show_section(title);

    let concepts = find_concepts_by_memory_type_fn(system, memory_type);

    if concepts.is_empty() {
        utils::show_info("Aucun concept trouvé");
    } else {
        for (i, id) in concepts.iter().enumerate() {
            println!("{}. {}", i + 1, id);
        }
        println!("Total: {} concepts", concepts.len());
    }
}

fn display_concepts_by_status<T>(
    system: &mut T,
    find_concepts_by_archival_status_fn: &impl Fn(&T, ArchivalStatus) -> Vec<String>,
    status: ArchivalStatus,
    title: &str,
) {
    utils::show_section(title);

    let concepts = find_concepts_by_archival_status_fn(system, status);

    if concepts.is_empty() {
        utils::show_info("Aucun concept trouvé");
    } else {
        for (i, id) in concepts.iter().enumerate() {
            println!("{}. {}", i + 1, id);
        }
        println!("Total: {} concepts", concepts.len());
    }
}

fn view_memory_transitions<T>(
    system: &mut T,
    get_concept_memory_transitions_fn: &impl Fn(
        &T,
        &str,
    )
        -> Result<Vec<(String, String, String, String)>, String>,
) {
    utils::show_section("Historique des Transitions de Mémoire");

    let id = utils::read_input("ID du concept");

    match get_concept_memory_transitions_fn(system, &id) {
        Ok(transitions) => {
            if transitions.is_empty() {
                utils::show_info("Aucune transition de mémoire pour ce concept");
            } else {
                println!("Historique des transitions pour '{}':", id);
                for (i, (timestamp, prev_type, new_type, cause)) in transitions.iter().enumerate() {
                    println!(
                        "{}. {} : {} -> {} (cause: {})",
                        i + 1,
                        timestamp,
                        prev_type,
                        new_type,
                        cause
                    );
                }
            }
        }
        Err(e) => utils::show_error(&e),
    }
}

fn create_memory_snapshot<T>(
    system: &mut T,
    create_memory_snapshot_fn: &impl Fn(&T, &str, &str) -> Result<String, String>,
) {
    utils::show_section("Créer un Snapshot de Mémoire");

    let directory = utils::read_input("Répertoire de sauvegarde (défaut: snapshots)");
    let directory = if directory.is_empty() {
        "snapshots".to_string()
    } else {
        directory
    };

    let description = utils::read_input("Description du snapshot");

    match create_memory_snapshot_fn(system, &directory, &description) {
        Ok(path) => utils::show_success(&format!("Snapshot créé avec succès: {}", path)),
        Err(e) => utils::show_error(&e),
    }
}

fn load_memory_snapshot<T>(
    system: &mut T,
    list_memory_snapshots_fn: &impl Fn(&T, &str) -> Result<Vec<String>, String>,
    load_memory_snapshot_fn: &impl Fn(&mut T, &str) -> Result<(), String>,
) {
    utils::show_section("Charger un Snapshot de Mémoire");

    let directory = utils::read_input("Répertoire des snapshots (défaut: snapshots)");
    let directory = if directory.is_empty() {
        "snapshots".to_string()
    } else {
        directory
    };

    match list_memory_snapshots_fn(system, &directory) {
        Ok(snapshots) => {
            if snapshots.is_empty() {
                utils::show_info("Aucun snapshot trouvé");
                return;
            }

            println!("Snapshots disponibles:");
            for (i, path) in snapshots.iter().enumerate() {
                println!("{}. {}", i + 1, path);
            }

            let choice = utils::read_input("Numéro du snapshot à charger (0 pour annuler)");

            match choice.parse::<usize>() {
                Ok(idx) if idx > 0 && idx <= snapshots.len() => {
                    let path = &snapshots[idx - 1];

                    if utils::confirm("Cette action remplacera la mémoire actuelle. Continuer?") {
                        match load_memory_snapshot_fn(system, path) {
                            Ok(_) => utils::show_success("Snapshot chargé avec succès"),
                            Err(e) => utils::show_error(&e),
                        }
                    }
                }
                Ok(0) => println!("Opération annulée"),
                _ => utils::show_error("Choix invalide"),
            }
        }
        Err(e) => utils::show_error(&e),
    }
}
