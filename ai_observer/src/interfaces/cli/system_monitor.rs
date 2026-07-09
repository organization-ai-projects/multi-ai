use crate::interfaces::cli::utils;
use std::io::{self, Write};

/// Affiche et gère le menu de monitoring du système
/// Utilise des fonctions de rappel pour éviter les dépendances directes
pub fn display_menu<T>(
    system: &mut T,
    // Fonctions déléguées qui seront fournies par l'appelant
    get_system_report: impl Fn(&T) -> String,
    get_recent_actions: impl Fn(&T, usize) -> Vec<(String, String, String, String)>, // timestamp, type, entités, résultat
    find_actions_for_concept: impl Fn(&T, &str) -> Vec<(String, String, String)>, // timestamp, type, résultat
    mut set_history_enabled: impl FnMut(&mut T, bool),
    mut clear_history: impl FnMut(&mut T),
    export_history: impl Fn(&T) -> Result<String, String>,
    is_history_enabled: impl Fn(&T) -> bool,
    mut set_execution_context: impl FnMut(&mut T, &str, Option<&str>, &str),
    mut add_context_tag: impl FnMut(&mut T, &str),
    mut set_context_property: impl FnMut(&mut T, &str, &str),
    mut clear_execution_context: impl FnMut(&mut T),
    run_task: impl Fn(&T, &str) -> Result<String, String>,
    list_available_tasks: impl Fn(&T) -> Vec<String>,
    start_background_task: impl Fn(&T, &str) -> Result<(), String>,
    stop_background_task: impl Fn(&T, &str) -> Result<(), String>,
    list_background_tasks: impl Fn(&T) -> Vec<(String, String)>,
    create_memory_snapshot: impl Fn(&T, &str, &str) -> Result<String, String>,
    get_concept_memory_transitions: impl Fn(
        &T,
        &str,
    )
        -> Result<Vec<(String, String, String, String)>, String>,
) {
    utils::show_section("Moniteur Système");

    loop {
        println!("1. Rapport d'introspection système");
        println!("2. Consulter l'historique des actions");
        println!("3. Gérer le contexte d'exécution");
        println!("4. Gérer les extensions et tâches");
        println!("5. Exporter les données système");
        println!("6. Voir les transitions de mémoire d'un concept");
        println!("7. Retour au menu principal");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => view_system_report(system, &get_system_report),
            2 => manage_action_history(
                system,
                &get_recent_actions,
                &find_actions_for_concept,
                &mut set_history_enabled,
                &mut clear_history,
                &export_history,
                &is_history_enabled,
            ),
            3 => manage_execution_context(
                system,
                &mut set_execution_context,
                &mut add_context_tag,
                &mut set_context_property,
                &mut clear_execution_context,
            ),
            4 => manage_extensions(
                system,
                &list_available_tasks,
                &run_task,
                &list_background_tasks,
                &start_background_task,
                &stop_background_task,
            ),
            5 => export_system_data(
                system,
                &get_system_report,
                &export_history,
                &create_memory_snapshot,
            ),
            6 => view_memory_transitions(system, &get_concept_memory_transitions),
            7 => break,
            _ => println!("Option invalide, veuillez réessayer."),
        }
    }
}

/// Affiche le rapport d'introspection système
fn view_system_report<T>(system: &T, get_system_report: &impl Fn(&T) -> String) {
    utils::show_section("Rapport d'Introspection Système");

    let report = get_system_report(system);
    println!("{}", report);

    // Attendre que l'utilisateur appuie sur Entrée pour continuer
    utils::read_input("Appuyez sur Entrée pour continuer");
}

/// Gère les fonctionnalités liées à l'historique des actions
fn manage_action_history<T>(
    system: &mut T,
    get_recent_actions: &impl Fn(&T, usize) -> Vec<(String, String, String, String)>,
    find_actions_for_concept: &impl Fn(&T, &str) -> Vec<(String, String, String)>,
    mut set_history_enabled: impl FnMut(&mut T, bool),
    mut clear_history: impl FnMut(&mut T),
    export_history: &impl Fn(&T) -> Result<String, String>,
    is_history_enabled_fn: &impl Fn(&T) -> bool,
) {
    utils::show_section("Gestion de l'Historique");

    loop {
        println!("1. Voir les dernières actions");
        println!("2. Rechercher les actions pour un concept");
        println!("3. Activer/désactiver l'historique");
        println!("4. Effacer l'historique");
        println!("5. Exporter l'historique (JSON)");
        println!("6. Retour au menu précédent");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => {
                utils::show_section("Dernières Actions");
                let limit =
                    utils::read_number::<usize>("Nombre d'actions à afficher").unwrap_or(10);
                let actions = get_recent_actions(system, limit);

                if actions.is_empty() {
                    utils::show_info("Aucune action enregistrée.");
                } else {
                    for (i, (timestamp, action_type, entities, result)) in
                        actions.iter().enumerate()
                    {
                        println!(
                            "{}. [{}] {} - Entités: {} - Résultat: {}",
                            i + 1,
                            timestamp,
                            action_type,
                            entities,
                            result
                        );
                    }
                }
            }
            2 => {
                utils::show_section("Recherche d'Actions par Concept");
                let concept_id = utils::read_input("ID du concept");
                let actions = find_actions_for_concept(system, &concept_id);

                if actions.is_empty() {
                    utils::show_info(&format!(
                        "Aucune action trouvée pour le concept '{}'",
                        concept_id
                    ));
                } else {
                    println!("Actions trouvées pour '{}':", concept_id);
                    for (i, (timestamp, action_type, result)) in actions.iter().enumerate() {
                        println!(
                            "{}. [{}] {} - Résultat: {}",
                            i + 1,
                            timestamp,
                            action_type,
                            result
                        );
                    }
                }
            }
            3 => {
                let enabled = utils::confirm("Activer l'historique des actions?");
                if enabled != is_history_enabled_fn(system) {
                    set_history_enabled(system, enabled);
                    utils::show_success(&format!(
                        "Historique des actions {}",
                        if enabled { "activé" } else { "désactivé" }
                    ));
                }
            }
            4 => {
                if utils::confirm("Êtes-vous sûr de vouloir effacer tout l'historique?") {
                    clear_history(system);
                    utils::show_success("Historique effacé");
                }
            }
            5 => {
                utils::show_section("Exportation de l'Historique");
                let path = utils::read_input(
                    "Chemin du fichier d'exportation (défaut: history_export.json)",
                );
                let path = if path.is_empty() {
                    "history_export.json".to_string()
                } else {
                    path
                };

                match export_history(system) {
                    Ok(json) => match std::fs::write(&path, json) {
                        Ok(_) => utils::show_success(&format!("Historique exporté vers {}", path)),
                        Err(e) => utils::show_error(&format!(
                            "Erreur lors de l'écriture du fichier: {}",
                            e
                        )),
                    },
                    Err(e) => utils::show_error(&e),
                }
            }
            6 => break,
            _ => println!("Option invalide"),
        }
    }
}

/// Gère le contexte d'exécution du système
fn manage_execution_context<T>(
    system: &mut T,
    set_execution_context_fn: &mut impl FnMut(&mut T, &str, Option<&str>, &str),
    add_context_tag_fn: &mut impl FnMut(&mut T, &str),
    set_context_property_fn: &mut impl FnMut(&mut T, &str, &str),
    clear_execution_context_fn: &mut impl FnMut(&mut T),
) {
    utils::show_section("Gestion du Contexte d'Exécution");

    loop {
        println!("1. Configurer un nouveau contexte");
        println!("2. Ajouter un tag au contexte");
        println!("3. Ajouter une propriété au contexte");
        println!("4. Effacer le contexte");
        println!("5. Retour au menu précédent");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => {
                let user_id = utils::read_input("Identifiant utilisateur");
                let session_id = utils::read_input_optional("Identifiant de session (optionnel)");
                let environment = utils::read_input("Environnement (dev, test, prod)");

                set_execution_context_fn(system, &user_id, session_id.as_deref(), &environment);
                utils::show_success("Contexte d'exécution configuré");
            }
            2 => {
                let tag = utils::read_input("Tag à ajouter");
                add_context_tag_fn(system, &tag);
                utils::show_success(&format!("Tag '{}' ajouté au contexte", tag));
            }
            3 => {
                let key = utils::read_input("Clé de la propriété");
                let value = utils::read_input("Valeur de la propriété");
                set_context_property_fn(system, &key, &value);
                utils::show_success(&format!("Propriété '{}' définie à '{}'", key, value));
            }
            4 => {
                clear_execution_context_fn(system);
                utils::show_success("Contexte d'exécution effacé");
            }
            5 => break,
            _ => println!("Option invalide"),
        }
    }
}

/// Gère les extensions et tâches d'arrière-plan
fn manage_extensions<T>(
    system: &T,
    list_available_tasks_fn: &impl Fn(&T) -> Vec<String>,
    run_task_fn: &impl Fn(&T, &str) -> Result<String, String>,
    list_background_tasks_fn: &impl Fn(&T) -> Vec<(String, String)>,
    start_background_task_fn: &impl Fn(&T, &str) -> Result<(), String>,
    stop_background_task_fn: &impl Fn(&T, &str) -> Result<(), String>,
) {
    utils::show_section("Gestion des Extensions");

    loop {
        println!("1. Lister les routines disponibles");
        println!("2. Exécuter une routine");
        println!("3. Lister les tâches d'arrière-plan");
        println!("4. Démarrer une tâche d'arrière-plan");
        println!("5. Arrêter une tâche d'arrière-plan");
        println!("6. Retour au menu précédent");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => {
                utils::show_section("Routines Disponibles");
                let routines = list_available_tasks_fn(system);

                if routines.is_empty() {
                    utils::show_info("Aucune routine n'est disponible");
                } else {
                    for (i, routine) in routines.iter().enumerate() {
                        println!("{}. {}", i + 1, routine);
                    }
                }
            }
            2 => {
                utils::show_section("Exécution de Routine");
                let name = utils::read_input("Nom de la routine à exécuter");

                match run_task_fn(system, &name) {
                    Ok(result) => utils::show_success(&format!("Routine exécutée: {}", result)),
                    Err(e) => utils::show_error(&e),
                }
            }
            3 => {
                utils::show_section("Tâches d'Arrière-Plan");
                let tasks = list_background_tasks_fn(system);

                if tasks.is_empty() {
                    utils::show_info("Aucune tâche d'arrière-plan n'est disponible");
                } else {
                    for (i, (name, status)) in tasks.iter().enumerate() {
                        println!("{}. {} - Statut: {}", i + 1, name, status);
                    }
                }
            }
            4 => {
                utils::show_section("Démarrage de Tâche");
                let name = utils::read_input("Nom de la tâche à démarrer");

                match start_background_task_fn(system, &name) {
                    Ok(_) => utils::show_success(&format!("Tâche '{}' démarrée", name)),
                    Err(e) => utils::show_error(&e),
                }
            }
            5 => {
                utils::show_section("Arrêt de Tâche");
                let name = utils::read_input("Nom de la tâche à arrêter");

                match stop_background_task_fn(system, &name) {
                    Ok(_) => utils::show_success(&format!("Tâche '{}' arrêtée", name)),
                    Err(e) => utils::show_error(&e),
                }
            }
            6 => break,
            _ => println!("Option invalide"),
        }
    }
}

/// Fonctionnalités d'exportation des données du système
fn export_system_data<T>(
    system: &mut T,
    get_system_report_fn: &impl Fn(&T) -> String,
    export_history_fn: &impl Fn(&T) -> Result<String, String>,
    create_memory_snapshot_fn: &impl Fn(&T, &str, &str) -> Result<String, String>,
) {
    utils::show_section("Exportation des Données Système");

    loop {
        println!("1. Exporter le rapport d'introspection");
        println!("2. Exporter l'historique des actions");
        println!("3. Créer un snapshot de la mémoire");
        println!("4. Retour au menu précédent");
        print!("Votre choix: ");
        io::stdout().flush().unwrap();

        match utils::read_menu_choice() {
            1 => {
                let path = utils::read_input(
                    "Chemin du fichier d'exportation (défaut: system_report.txt)",
                );
                let path = if path.is_empty() {
                    "system_report.txt".to_string()
                } else {
                    path
                };

                let report = get_system_report_fn(system);
                match std::fs::write(&path, report) {
                    Ok(_) => utils::show_success(&format!("Rapport exporté vers {}", path)),
                    Err(e) => {
                        utils::show_error(&format!("Erreur lors de l'écriture du fichier: {}", e))
                    }
                }
            }
            2 => {
                let path = utils::read_input(
                    "Chemin du fichier d'exportation (défaut: action_history.json)",
                );
                let path = if path.is_empty() {
                    "action_history.json".to_string()
                } else {
                    path
                };

                match export_history_fn(system) {
                    Ok(json) => match std::fs::write(&path, json) {
                        Ok(_) => utils::show_success(&format!("Historique exporté vers {}", path)),
                        Err(e) => utils::show_error(&format!(
                            "Erreur lors de l'écriture du fichier: {}",
                            e
                        )),
                    },
                    Err(e) => utils::show_error(&e),
                }
            }
            3 => {
                let directory = utils::read_input("Répertoire de destination (défaut: snapshots)");
                let directory = if directory.is_empty() {
                    "snapshots"
                } else {
                    &directory
                };
                let description = utils::read_input("Description du snapshot");

                match create_memory_snapshot_fn(system, directory, &description) {
                    Ok(path) => {
                        utils::show_success(&format!("Snapshot créé avec succès: {}", path))
                    }
                    Err(e) => utils::show_error(&e),
                }
            }
            4 => break,
            _ => println!("Option invalide"),
        }
    }
}

/// Affiche l'historique des transitions de mémoire pour un concept
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

    // Afficher d'abord les transitions de mémoire
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
