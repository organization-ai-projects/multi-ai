use crate::entities::memory::graph_memory::GraphMemory;
use crate::entities::memory::managers::{
    concept_manager::ConceptManager, graph_manager::GraphManager, persistence_manager::PersistenceManager,
    query_manager::QueryManager, relation_manager::RelationManager, statistics_manager::StatisticsManager,
};
use std::collections::HashMap;

/// Orchestrateur principal du système d'IA
pub struct AISystemOrchestrator {
    graph_manager: GraphManager,
    statistics_manager: StatisticsManager,
    query_manager: QueryManager,
    persistence_manager: PersistenceManager,
    relation_manager: RelationManager,
    storage_path: String,
}

impl AISystemOrchestrator {
    /// Initialise l'orchestrateur avec le chemin de stockage spécifié
    pub fn initialize(storage_path: &str) -> Self {
        // Charger ou créer le graphe de connaissances
        let memory = match GraphMemory::load_memory(storage_path) {
            Ok(mem) => {
                println!("Mémoire d'IA chargée avec succès");
                println!(
                    "Version: {}, Concepts: {}, Relations: {}",
                    mem.get_version(),
                    mem.graph.node_count(),
                    mem.graph.edge_count()
                );
                mem
            }
            Err(e) => {
                println!("Création d'une nouvelle mémoire d'IA: {}", e);
                GraphMemory::new(0.005)
            }
        };

        // Créer un GraphManager avec la mémoire chargée ou créée
        let mut graph_manager = GraphManager::new(0.005);
        // Remplacer la mémoire par défaut par celle chargée
        *graph_manager.get_memory_mut() = memory;

        AISystemOrchestrator {
            graph_manager,
            statistics_manager: StatisticsManager,
            query_manager: QueryManager,
            persistence_manager: PersistenceManager,
            relation_manager: RelationManager,
            storage_path: storage_path.to_string(),
        }
    }

    /// Initialise l'orchestrateur avec une mémoire existante
    pub fn with_memory(memory: GraphMemory, storage_path: &str) -> Self {
        let mut graph_manager = GraphManager::new(0.005);
        // Remplacer la mémoire par défaut par celle fournie
        *graph_manager.get_memory_mut() = memory;

        AISystemOrchestrator {
            graph_manager,
            statistics_manager: StatisticsManager,
            query_manager: QueryManager,
            persistence_manager: PersistenceManager,
            relation_manager: RelationManager,
            storage_path: storage_path.to_string(),
        }
    }

    /// Récupère une référence à la mémoire de l'IA
    pub fn get_memory(&self) -> &GraphMemory {
        self.graph_manager.get_memory()
    }

    /// Récupère une référence mutable à la mémoire
    pub fn get_memory_mut(&mut self) -> &mut GraphMemory {
        self.graph_manager.get_memory_mut()
    }

    /// Sauvegarde l'état du système
    pub fn save(&self) -> Result<(), String> {
        self.persistence_manager
            .save(self.graph_manager.get_memory(), &self.storage_path)
    }

    /// Obtient le chemin de stockage
    pub fn get_storage_path(&self) -> String {
        self.storage_path.clone()
    }

    /// Ajoute un concept à la mémoire
    pub fn add_concept(&mut self, id: String, content: String, metadata: HashMap<String, String>) {
        self.graph_manager.add_node(id, content, metadata).unwrap();
    }

    /// Archive un concept
    pub fn archive_concept(&mut self, id: &str) -> Result<(), String> {
        self.graph_manager.archive_node(id)
    }

    /// Désarchive un concept
    pub fn unarchive_concept(&mut self, id: &str) -> Result<(), String> {
        self.graph_manager.unarchive_node(id)
    }

    /// Crée une relation entre deux concepts
    pub fn connect_concepts(&mut self, from_id: &str, to_id: &str, relation_type: String) -> Result<(), String> {
        RelationManager::connect_nodes(
            self.graph_manager.get_memory_mut(),
            from_id,
            to_id,
            relation_type
        )
    }

    /// Crée une relation avec des tags entre deux concepts
    pub fn connect_concepts_with_tags(
        &mut self,
        from_id: &str,
        to_id: &str,
        relation_type: String,
        tags: Vec<String>,
    ) -> Result<(), String> {
        RelationManager::connect_nodes_with_tags(
            self.graph_manager.get_memory_mut(),
            from_id,
            to_id,
            relation_type,
            tags
        )
    }

    /// Expose un concept pour augmenter son importance
    pub fn expose_concept(&mut self, id: &str) -> Result<(), String> {
        ConceptManager::expose_node(self.graph_manager.get_memory_mut(), id)
    }

    /// Tague un groupe de concepts
    pub fn tag_concept_cluster(&mut self, ids: &[&str], tag: &str) -> Result<(), String> {
        ConceptManager::tag_conceptual_cluster(self.graph_manager.get_memory_mut(), ids, tag)
    }

    /// Promote un concept vers la mémoire
    pub fn promote_concept_memory(&mut self, id: &str, cause: &str) -> Result<bool, String> {
        self.graph_manager.promote_node(id, cause)
    }

    /// Trouve des concepts par type de mémoire
    pub fn find_concepts_by_memory_type(&self, memory_type: &str) -> Vec<String> {
        // Encapsulation complète - le type interne est masqué
        match memory_type {
            "STM" => self.graph_manager.find_nodes_by_stm(),
            "MTM" => self.graph_manager.find_nodes_by_mtm(),
            "LTM" => self.graph_manager.find_nodes_by_ltm(),
            _ => vec![],
        }
    }

    /// Trouve des concepts par statut d'archivage
    pub fn find_concepts_by_archival_status(&self, status: &str) -> Vec<String> {
        // Encapsulation complète - le statut interne est masqué
        match status {
            "Active" => self.graph_manager.find_active_nodes(),
            "Archived" => self.graph_manager.find_archived_nodes(),
            _ => vec![],
        }
    }

    /// Récupère les statistiques de mémoire
    pub fn get_memory_statistics(&self) -> HashMap<String, String> {
        StatisticsManager::calculate_statistics(self.graph_manager.get_memory())
    }

    /// Trouve le chemin le plus court entre deux concepts
    pub fn find_shortest_path(&self, from_id: &str, to_id: &str) -> Result<Vec<String>, String> {
        self.query_manager
            .find_shortest_path(self.graph_manager.get_memory(), from_id, to_id)
    }
}
