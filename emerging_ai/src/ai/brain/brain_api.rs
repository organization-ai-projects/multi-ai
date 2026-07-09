use tokio::sync::broadcast;
use crate::brain::{cache::CacheManager, critic::CriticManager, goal::GoalManager};
use serde::{Deserialize, Serialize};
use std::any::Any;
use uuid::Uuid;

pub struct BrainAPI {
    neural_watcher: NeuralWatcher,
    cache_manager: CacheManager,
    critic_manager: CriticManager,
    goal_manager: GoalManager,
    signal_tx: broadcast::Sender<NeuralSignal>,
}

impl BrainAPI {
    pub fn new() -> Self {
        let (tx, rx) = broadcast::channel(100);
        let neural_watcher = NeuralWatcher::new(rx);

        Self {
            neural_watcher,
            signal_tx: tx,
            cache_manager: CacheManager::new(),
            critic_manager: CriticManager::new(),
            goal_manager: GoalManager::new(),
        }
    }

    // Toutes les méthodes devraient uniquement déléguer aux managers
    pub fn begin_cycle(&mut self) -> std::io::Result<Uuid> {
        let cycle_id = Uuid::now_v7();
        self.goal_manager.start_cycle(cycle_id)
    }

    pub fn record_learning(&mut self, success: bool, score: f32) {
        self.critic_manager.record_result(success, score);
    }

    pub fn suggest_next_goal(&self) -> &'static str {
        self.critic_manager.suggest_next_goal()
    }

    // Délégation simple au GoalManager
    pub fn add_goal(&mut self, goal_type: &str, priority: u8) -> Uuid {
        self.goal_manager.add_goal(goal_type, priority)
    }

    pub fn update_goal_progress(&mut self, uuid: Uuid, progress: f32) -> bool {
        self.goal_manager.update_progress(uuid, progress)
    }

    pub fn get_current_goal(&self) -> Option<Uuid> {
        self.goal_manager.get_next_goal()
    }

    pub fn abandon_goal(&mut self, uuid: Uuid) {
        self.goal_manager.abandon_goal(uuid)
    }

    // Configuration
    pub fn set_read_only(&self, value: bool) {
        self.cache_manager.set_read_only(value);
    }

    pub fn print_stats(&self) {
        self.cache_manager.print_stats();
    }

    // Operations standards via route
    pub fn get_node(&mut self, uuid: Uuid) -> std::io::Result<Option<&dyn std::fmt::Debug>> {
        self.cache_manager.route("node", "get", uuid)
    }

    pub fn save_node(&mut self, uuid: Uuid, data: &[u8]) -> std::io::Result<()> {
        self.cache_manager.route("node", "save", (uuid, data))
    }

    pub fn list_nodes(&self) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route("node", "list", ())
    }

    pub fn get_link(&mut self, uuid: Uuid) -> std::io::Result<Option<&dyn std::fmt::Debug>> {
        self.cache_manager.route("link", "get", uuid)
    }

    pub fn save_link(&mut self, uuid: Uuid, data: &[u8]) -> std::io::Result<()> {
        self.cache_manager.route("link", "save", (uuid, data))
    }

    pub fn list_links(&self) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route("link", "list", ())
    }

    pub fn get_episode(&mut self, uuid: Uuid) -> std::io::Result<Option<&dyn std::fmt::Debug>> {
        self.cache_manager.route("episode", "get", uuid)
    }

    pub fn save_episode(&mut self, uuid: Uuid, data: &[u8]) -> std::io::Result<()> {
        self.cache_manager.route("episode", "save", (uuid, data))
    }

    pub fn list_episodes(&self) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route("episode", "list", ())
    }

    pub fn list_logs(&self) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route("log", "list", ())
    }

    pub fn list_images(&self) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route("image", "list", ())
    }

    pub fn list_codes(&self) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route("code", "list", ())
    }

    // Standard operations via route
    pub fn get_code(&mut self, uuid: Uuid) -> std::io::Result<Option<&dyn std::fmt::Debug>> {
        self.cache_manager.route("code", "get", uuid)
    }

    pub fn save_code(&mut self, uuid: Uuid, data: &[u8]) -> std::io::Result<()> {
        self.cache_manager.route("code", "save", (uuid, data))
    }

    // Legacy methods
    pub fn save_code_legacy<T: serde::Serialize>(&mut self, code: T) -> std::io::Result<()> {
        self.cache_manager.route("code", "save_legacy", code)
    }

    pub fn save_node_legacy<T: serde::Serialize>(&mut self, node: T) -> std::io::Result<()> {
        self.cache_manager.route("node", "save_legacy", node)
    }

    pub fn save_link_legacy<T: serde::Serialize>(&mut self, link: T) -> std::io::Result<()> {
        self.cache_manager.route("link", "save_legacy", link)
    }

    pub fn save_episode_legacy<T: serde::Serialize>(&mut self, episode: T) -> std::io::Result<()> {
        self.cache_manager.route("episode", "save_legacy", episode)
    }

    pub fn save_log_legacy<T: serde::Serialize>(&mut self, log: T) -> std::io::Result<()> {
        self.cache_manager.route("log", "save_legacy", log)
    }

    pub fn save_image_legacy<T: serde::Serialize>(&mut self, image: T) -> std::io::Result<()> {
        self.cache_manager.route("image", "save_legacy", image)
    }

    // Recherches avancées
    pub fn find_nodes_modified_after(&self, timestamp: u64) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.find_nodes_modified_after(timestamp)
    }

    pub fn find_links_by_node(&self, node_uuid: Uuid) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.find_links_by_node(node_uuid)
    }

    // Batch operations
    pub fn batch_save(&self, entity_type: &str, uuids: &[Uuid]) -> std::io::Result<()> {
        self.cache_manager.route(entity_type, "batch_save", uuids)
    }

    pub fn batch_load(&self, entity_type: &str, uuids: &[Uuid]) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.route(entity_type, "batch_load", uuids)
    }

    // Méthodes pour le traitement par lots
    pub fn batch_save_nodes(&self, uuids: &[Uuid], data: &[&[u8]]) -> std::io::Result<()> {
        for (uuid, bytes) in uuids.iter().zip(data.iter()) {
            self.cache_manager.route("node", "save", (*uuid, *bytes))?;
        }
        Ok(())
    }

    pub fn batch_load_nodes(&mut self, uuids: &[Uuid]) -> std::io::Result<Vec<Option<Vec<u8>>>> {
        let mut results = Vec::new();
        for uuid in uuids {
            let result = self.cache_manager.route("node", "get", *uuid)?;
            results.push(result);
        }
        Ok(results)
    }

    pub fn batch_save_links(&self, uuids: &[Uuid], data: &[&[u8]]) -> std::io::Result<()> {
        for (uuid, bytes) in uuids.iter().zip(data.iter()) {
            self.cache_manager.route("link", "save", (*uuid, *bytes))?;
        }
        Ok(())
    }

    pub fn batch_load_links(&mut self, uuids: &[Uuid]) -> std::io::Result<Vec<Option<Vec<u8>>>> {
        let mut results = Vec::new();
        for uuid in uuids {
            let result = self.cache_manager.route("link", "get", *uuid)?;
            results.push(result);
        }
        Ok(results)
    }

    // Méthodes pour les recherches avancées
    pub fn find_nodes_modified_after(&self, timestamp: u64) -> std::io::Result<Vec<Self::Node>> {
        self.cache_manager.find_nodes_modified_after(timestamp)
    }

    pub fn find_links_by_node(&self, node_uuid: Uuid) -> std::io::Result<Vec<Self::Link>> {
        let all_links = self.list_all_links()?;
        Ok(all_links
            .into_iter()
            .filter(|link| link.source == node_uuid || link.target == node_uuid)
            .collect())
    }

    // Méthodes pour la journalisation avancée - délégation au cache_manager
    pub fn save_with_journal(&mut self, uuid: Uuid, entity_type: &str) -> std::io::Result<()> {
        self.cache_manager.save_with_journal(uuid, entity_type)
    }

    pub fn restore_from_journal(&mut self) -> std::io::Result<()> {
        self.cache_manager.restore_from_journal()
    }

    // Indexation - délégation au cache_manager
    pub fn update_index(&mut self, uuid: Uuid, timestamp: u64, entity_type: &str) {
        self.cache_manager
            .update_index(uuid, timestamp, entity_type)
    }

    // Les méthodes spécifiques à node/link deviennent :
    pub fn save_node_with_logging(&mut self, uuid: Uuid) -> std::io::Result<()> {
        self.cache_manager.save_node_with_logging(uuid)
    }

    pub fn save_link_with_logging(&mut self, uuid: Uuid) -> std::io::Result<()> {
        self.cache_manager.save_link_with_logging(uuid)
    }

    /// Applique une stratégie de parcours et retourne les UUIDs des entités parcourues
    pub fn traverse(&mut self, strategy: &str) -> std::io::Result<Vec<Uuid>> {
        self.cache_manager.traverse(strategy)
    }

    // Intégration avec le critic
    pub fn evaluate_code(&self, code: &str) -> f32 {
        self.critic_manager.evaluate_code(code)
    }

    pub fn get_code_complexity(&self, code: &str) -> f32 {
        self.critic_manager.calculate_complexity(code)
    }

    pub fn evaluate_mutation(&self, original: &str, mutated: &str) -> f32 {
        self.critic_manager.evaluate_mutation(original, mutated)
    }

    pub fn begin_transaction(&mut self) -> std::io::Result<()> {
        self.cache_manager.begin_transaction()
    }

    pub fn commit_transaction(&mut self) -> std::io::Result<()> {
        self.cache_manager.commit_transaction()
    }

    pub fn rollback_transaction(&mut self) -> std::io::Result<()> {
        self.cache_manager.rollback_transaction()
    }

    pub fn count_artifacts(&self) -> u32 {
        // Compte tous les artéfacts (nodes, links, codes, etc.)
        self.cache_manager.count_all_artifacts()
    }

    pub fn is_in_emergency(&self) -> bool {
        self.critic_manager.is_in_emergency()
    }

    pub fn handle_critical_error(&mut self, error: &str) -> bool {
        self.critic_manager.handle_critical_error(error)
    }

    pub fn should_attempt_mutation(&self, code: &str, score: f32) -> bool {
        // Laisser l'IA décider quand muter en fonction de ses critères
        self.critic_manager
            .evaluate_mutation_opportunity(code, score)
    }

    pub fn process_mutation_result(&mut self, original: &str, mutated: &str, score: f32) {
        self.critic_manager
            .learn_from_mutation(original, mutated, score);
    }

    pub fn should_generate_new_goal(&self) -> bool {
        self.goal_manager.evaluate_goal_generation_need()
    }

    pub fn generate_goal(&mut self) -> Uuid {
        self.goal_manager.generate_goal()
    }

    pub fn should_explore_path(&self, path: &str) -> bool {
        // Utilise l'historique d'exploration et les résultats passés
        self.goal_manager.evaluate_exploration_value(path)
    }

    pub fn analyze_code_context(&self, code: &str, location: &str) -> CodeMetrics {
        self.critic_manager.analyze_full_context(code, location)
    }

    pub fn get_recommended_strategies(&self, code: &str) -> Vec<Strategy> {
        self.critic_manager.recommend_mutation_strategies(code)
    }

    pub fn learn_from_mutation_result(&mut self, original: &str, mutated: &str, score: f32) {
        self.critic_manager
            .record_mutation_result(original, mutated, score);
        self.goal_manager.update_learning_progress(score);
    }

    pub fn save_successful_mutation(
        &mut self,
        original: &str,
        mutated: &str,
        score: f32,
        strategy: &Strategy,
    ) -> std::io::Result<()> {
        let mutation_id = Uuid::now_v7();
        self.cache_manager
            .save_mutation(mutation_id, original, mutated, score, strategy)
    }

    pub fn should_adjust_goals(&self) -> bool {
        self.goal_manager.needs_goal_adjustment()
    }

    pub fn evolve_goals(&mut self) -> std::io::Result<()> {
        self.goal_manager.evolve_goals_based_on_learning()
    }

    // La mémoire se gère automatiquement via des traits
    pub fn record<T: Observable>(&mut self, item: &T) -> std::io::Result<()> {
        let id = Uuid::now_v7();
        item.observe(id)?;
        Ok(())
    }

    // Permet à l'IA d'enregistrer n'importe quelle observation
    pub fn record_observation(
        &mut self,
        context: &str,
        value: Observation,
    ) -> std::io::Result<Uuid> {
        let id = Uuid::now_v7();

        // On ne rejette jamais une observation, même inattendue
        self.cache_manager
            .store_observation(id, context, value, None)?;

        // On laisse l'IA créer ses propres connections
        if let Some(pattern) = self.discover_pattern(context, &value) {
            self.cache_manager.store_pattern(pattern)?;
        }

        Ok(id)
    }

    // L'IA peut créer ses propres associations
    pub fn associate(
        &mut self,
        from: Uuid,
        to: Uuid,
        relation: Option<String>,
    ) -> std::io::Result<()> {
        self.cache_manager.create_association(from, to, relation)
    }

    fn discover_pattern(&self, context: &str, value: &Observation) -> Option<Pattern> {
        // L'IA analyse librement les observations et trouve des patterns
        // ...
        None
    }

    // Le cerveau réagit automatiquement aux événements
    fn on_signal(&mut self, signal: impl Into<NeuralSignal>) {
        self.neural_watcher.sense(signal.into());
        // Le reste est automatique
    }

    fn process_signal<T: Into<NeuralSignal>>(&mut self, input: T) {
        // Le cerveau réagit naturellement aux signaux
        self.neural_watcher.sense(input.into());
    }
}

// Structure flexible pour stocker n'importe quel type d'observation
pub enum Observation {
    Number(f64),
    Text(String),
    Boolean(bool),
    List(Vec<Observation>),
    Map(std::collections::HashMap<String, Observation>),
    // Pour les associations inattendues
    Mixed(Box<dyn Any>),
}
