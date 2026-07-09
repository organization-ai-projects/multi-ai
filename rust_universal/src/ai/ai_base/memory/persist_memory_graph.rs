// Façade unique pour manipuler et persister la mémoire graphique de l'IA.

use super::memory_graph::MemoryGraph;
use super::persist::{self, ExtensionManager};
use super::memory_graph::MemoryTerm;

pub const DEFAULT_GRAPH_PATH: &str = "memory_graph"; // <- Supprime l'extension

// Alias pour simplifier les types de résultats
type Result<T> = std::io::Result<T>;

pub struct PersistentMemoryGraph {
    graph: MemoryGraph,          // Champ encapsulé pour manipuler le graphe
    manager: ExtensionManager,   // Gestionnaire d'extensions interne
}

impl PersistentMemoryGraph {
    pub fn new() -> Self {
        Self {
            graph: MemoryGraph::new(),
            manager: ExtensionManager::new(), // Initialise un gestionnaire par défaut
        }
    }

    pub fn add_node<S: Into<String>>(&mut self, id: usize, data: S) {
        self.graph.add_node(id, data);
    }

    pub fn add_nodes<I, S>(&mut self, nodes: I)
    where
        I: IntoIterator<Item = (usize, S)>,
        S: Into<String>,
    {
        self.graph.add_nodes(nodes);
    }

    pub fn add_link(&mut self, from: usize, to: usize, weight: f32) {
        self.graph.add_link(from, to, weight);
    }

    pub fn add_links<I>(&mut self, links: I)
    where
        I: IntoIterator<Item = (usize, usize, f32)>,
    {
        self.graph.add_links(links);
    }

    /// Ajoute un lien bidirectionnel entre deux nœuds.
    pub fn add_bidirectional_link(&mut self, from: usize, to: usize, weight: f32) {
        self.graph.add_bidirectional_link(from, to, weight);
    }

    pub fn is_valid(&self) -> bool {
        self.graph.is_valid()
    }

    /// Stocke une valeur dans la mémoire à court terme.
    pub fn store_short_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.graph.store_short_term(key, value);
    }

    /// Stocke une valeur dans la mémoire à moyen terme.
    pub fn store_medium_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.graph.store_medium_term(key, value);
    }

    /// Stocke une valeur dans la mémoire à long terme.
    pub fn store_long_term(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.graph.store_long_term(key, value);
    }

    /// Récupère une valeur de la mémoire à court terme.
    pub fn retrieve_short_term(&self, key: &str) -> Option<&String> {
        self.graph.retrieve_short_term(key)
    }

    /// Récupère une valeur de l'archive de la mémoire à court terme.
    pub fn retrieve_archived_short_term(&self, key: &str) -> Option<&String> {
        self.graph.retrieve_archived_short_term(key)
    }

    /// Résout le chemin en utilisant DEFAULT_GRAPH_PATH si aucun chemin n'est fourni.
    fn resolve_path<'a>(path: Option<&'a str>) -> &'a str {
        path.unwrap_or(DEFAULT_GRAPH_PATH)
    }

    /// Sauvegarde le graphe mémoire dans un fichier.
    /// Le format est automatiquement géré par `persist::save_auto`.
    pub fn save(&self, ia_id: &str) -> Result<()> {
        let path = format!("{}/{}", ia_id, DEFAULT_GRAPH_PATH);
        persist::save_auto(&self.graph, &path, &self.manager)
    }

    /// Charge un graphe mémoire depuis un fichier et remplace le graphe actuel.
    pub fn load(&mut self, ia_id: &str) -> Result<()> {
        let path = format!("{}/{}", ia_id, DEFAULT_GRAPH_PATH);
        let graph = persist::auto_load::<MemoryGraph>(&path, &self.manager)?;
        if !graph.is_valid() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid graph",
            ));
        }
        self.graph = graph; // Remplace le graphe actuel
        Ok(())
    }

    /// Ajoute ou met à jour une donnée dans la mémoire avec un type spécifique.
    pub fn store_memory(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.graph.store_memory(key, value, MemoryTerm::ShortTerm);
    }

    /// Récupère une donnée en cherchant en priorité dans la mémoire à court terme, puis moyen, puis long terme.
    pub fn retrieve_memory(&self, key: &str) -> Option<&String> {
        self.graph.retrieve_memory(key)
    }

    /// Met à jour la donnée en mémoire courte
    pub fn update_to_short_term(&mut self, key: &str) {
        self.graph.update_to_short_term(key);
    }

    /// Met à jour la donnée en mémoire moyenne
    pub fn update_to_medium_term(&mut self, key: &str) {
        self.graph.update_to_medium_term(key);
    }

    /// Met à jour la donnée en mémoire longue
    pub fn update_to_long_term(&mut self, key: &str) {
        self.graph.update_to_long_term(key);
    }

    /// Liste toutes les données en mémoire courte
    pub fn list_short_term(&self) -> Vec<&String> {
        self.graph.list_short_term()
    }

    /// Liste toutes les données en mémoire moyenne
    pub fn list_medium_term(&self) -> Vec<&String> {
        self.graph.list_medium_term()
    }

    /// Liste toutes les données en mémoire longue
    pub fn list_long_term(&self) -> Vec<&String> {
        self.graph.list_long_term()
    }

    /// Déplace toutes les données de la mémoire courte vers la mémoire moyenne
    pub fn promote_short_term_to_medium(&mut self) {
        self.graph.promote_short_term_to_medium();
    }

    /// Vérifie si un lien existe entre deux nœuds.
    pub fn link_exists(&self, from: usize, to: usize) -> bool {
        self.graph.link_exists(from, to)
    }

    /// Ajuste le poids d'un lien existant ou le crée si nécessaire.
    pub fn adjust_link_weight(&mut self, from: usize, to: usize, delta: f32) {
        self.graph.links.adjust_link_weight(from, to, delta);
    }

    /// Récupère le poids d'un lien entre deux nœuds.
    pub fn get_link_weight(&self, from: usize, to: usize) -> Option<f32> {
        self.graph.links.get_link_weight(from, to)
    }
}
