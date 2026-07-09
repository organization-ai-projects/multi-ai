//il est le seul dans cli/ à connaitre api/ au travers de system_api.rs
use crate::api::system_api::SystemAPI;
use crate::interfaces::cli::{
    agent_coordinator, concept_editor, knowledge_explorer, memory_manager, relation_manager,
    system_monitor, utils,
};
use std::io::{self, Write};

/// Interface en ligne de commande
/// IMPORTANT: cli_interface est le seul module dans cli/ qui peut dépendre de api/system_api
pub struct CLIInterface {
    api: SystemAPI,
}

impl CLIInterface {
    /// Crée une nouvelle interface CLI en utilisant l'API fournie
    pub fn new(api: SystemAPI) -> Self {
        CLIInterface { api }
    }

    /// Exécute l'interface CLI
    pub fn run(&mut self) {
        println!("Interface en ligne de commande démarrée");

        // Utiliser l'API système directement
        self.api
            .initialize_default_context("cli_user", "development");

        loop {
            match self.display_main_menu() {
                1 => self.run_concept_menu(),
                2 => self.run_relation_menu(),
                3 => self.run_query_menu(),
                4 => self.run_agent_coordinator_menu(),
                5 => self.run_memory_manager_menu(),
                6 => self.run_system_monitor_menu(),
                7 => {
                    // Sauvegarder et quitter
                    if let Err(e) = self.api.save() {
                        utils::show_error(&format!("Erreur lors de la sauvegarde: {}", e));
                    } else {
                        utils::show_success("Système d'IA sauvegardé. Au revoir!");
                    }
                    break;
                }
                _ => utils::show_error("Option invalide, veuillez réessayer."),
            }
        }
    }

    /// Affiche le menu principal et retourne le choix utilisateur
    fn display_main_menu(&self) -> u32 {
        utils::show_section("Menu Principal");
        println!("1. Éditeur de concepts");
        println!("2. Gestionnaire de relations");
        println!("3. Explorateur de connaissances");
        println!("4. Coordinateur d'agents IA");
        println!("5. Gestionnaire de mémoire");
        println!("6. Moniteur système");
        println!("7. Sauvegarder et quitter");
        print!("\nVotre choix: ");
        io::stdout().flush().unwrap();

        utils::read_menu_choice()
    }

    // ===== Lancement des menus spécifiques =====

    /// Exécute le menu de coordination des agents
    fn run_agent_coordinator_menu(&mut self) {
        agent_coordinator::display_menu(
            &mut self.api,
            |api, agent_id| api.create_agent(agent_id),
            |api, agent_id, concept_id, content, metadata| {
                api.create_concept_in_agent(agent_id, concept_id, content, metadata)
            },
            |api, agent_id, concept_id| api.share_concept(agent_id, concept_id),
            |api, concept_id| api.broadcast_concept(concept_id),
            |api| api.get_sharing_stats(),
        );
    }

    /// Exécute le menu d'édition de concepts
    fn run_concept_menu(&mut self) {
        concept_editor::display_menu(
            &mut self.api,
            |api, id, content, metadata| api.create_concept(id, content, metadata),
            |api, id, content, metadata, importance| {
                api.create_concept_with_importance(id, content, metadata, importance)
            },
            |api| api.get_all_concepts(),
            |api, ids, tag| api.tag_concepts(ids, tag),
            |api, id| api.expose_concept(id),
            |api, id, importance| api.set_concept_importance(id, importance),
            |api, ids, tag| api.tag_concepts(ids, tag),
        );
    }

    /// Exécute le menu de gestion des relations
    fn run_relation_menu(&mut self) {
        relation_manager::display_menu(
            &mut self.api,
            |api, from_id, to_id, relation_type| api.create_relation(from_id, to_id, relation_type),
            |api, from_id, to_id, tag| api.add_relation_tag(from_id, to_id, tag),
            |api, from_id, to_id, success| api.record_relation_activation(from_id, to_id, success),
            |api| api.get_all_relations(),
        );
    }

    /// Exécute le menu d'exploration des connaissances
    fn run_query_menu(&mut self) {
        knowledge_explorer::display_menu(
            &mut self.api,
            |api, id, limit| api.find_related_concepts(id, limit),
            |api, from_id, to_id| api.find_path(from_id, to_id),
            |api, tag| api.find_concepts_by_tag(tag),
            |api, tag| api.find_relations_by_tag(tag),
            |api| api.get_system_stats(),
            |api, id| api.get_concept_weight(id),
        );
    }

    /// Exécute le menu de gestion de la mémoire
    fn run_memory_manager_menu(&mut self) {
        memory_manager::display_menu(
            &mut self.api,
            |api, id| api.expose_concept(id),
            |api, id| api.archive_concept(id),
            |api, id| api.unarchive_concept(id),
            |api, id, cause| api.promote_concept_memory(id, cause),
            |api| api.get_memory_stats().to_string(),
            |api, memory_type| api.find_concepts_by_memory_type(memory_type),
            |api, status| api.find_concepts_by_archival_status(status),
            |api, id| api.get_concept_memory_transitions(id),
            |api, dir, desc| api.create_memory_snapshot(dir, desc),
            |api, dir| api.list_memory_snapshots(dir),
            |api, path| api.load_memory_snapshot(path),
        );
    }

    /// Exécute le menu de monitoring système
    fn run_system_monitor_menu(&mut self) {
        system_monitor::display_menu(
            &mut self.api,
            |api| api.get_system_report(),
            |api, limit| {
                api.get_history(limit)
                    .into_iter()
                    .map(|r| {
                        (
                            r.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                            r.action_type.to_string(),
                            r.entity_ids.join(", "),
                            r.result.to_string(),
                        )
                    })
                    .collect()
            },
            |api, concept_id| {
                api.find_actions_for_concept(concept_id)
                    .into_iter()
                    .map(|r| {
                        (
                            r.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
                            r.action_type.to_string(),
                            r.result.to_string(),
                        )
                    })
                    .collect()
            },
            |api, enabled| api.set_history_enabled(enabled),
            |api| api.clear_history(),
            |api| api.export_history(),
            |api| api.is_history_enabled(),
            |api, user_id, session_id, environment| {
                api.initialize_context(user_id, session_id, environment);
            },
            |api, tag| api.add_context_tag(tag),
            |api, key, value| api.set_context_property(key, value),
            |api| api.clear_context(),
            |api, name| api.run_task(name),
            |api| api.list_available_tasks(),
            |api, name| api.start_background_task(name),
            |api, name| api.stop_background_task(name),
            |api| api.list_background_tasks(),
            |api, dir, desc| api.create_memory_snapshot(dir, desc),
            |api, id| {
                // Ajout de la fonction manquante pour les transitions de mémoire
                api.get_memory_manager()
                    .get_concept_memory_transitions(id)
                    .map(|transitions| {
                        transitions
                            .into_iter()
                            .map(|t| {
                                (
                                    t.timestamp.to_string(),
                                    t.previous_type.to_string(),
                                    t.new_type.to_string(),
                                    t.cause,
                                )
                            })
                            .collect()
                    })
            },
        );
    }
}
