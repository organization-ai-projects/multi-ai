use crate::brain::memory::{MemoryGraph, MemoryNode, MemoryEdge, FileExtensionInfo};
use crate::brain::memory_episode::MemoryEpisode; // Utilisation interne uniquement
use crate::brain::memory_artifact::{MemoryImage, MemoryCode, MemoryLog};
use crate::capacities::mutate_ast::Strategy;
use std::fs::{File, OpenOptions};
use std::io::Write;
use serde::{Serialize, Deserialize};
use bincode; // Import pour la sérialisation binaire
use ron; // Import pour la sérialisation RON
use std::collections::HashMap;
use uuid::Uuid;
use rand::seq::IteratorRandom;

mod persistents;

pub use persistents::nodes::*;
pub use persistents::episodes::*;
pub use persistents::artifacts::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistentMemoryGraph {
    pub nodes: HashMap<Uuid, MemoryNode>,
    pub edges: Vec<(Uuid, Uuid, MemoryEdge)>,
    pub file_extensions: HashMap<String, FileExtensionInfo>,
    pub episodes: Vec<MemoryEpisode>, // Liste des épisodes
}

impl PersistentMemoryGraph {
    pub fn from_memory_graph(memory: &MemoryGraph) -> Self {
        PersistentMemoryGraph {
            nodes: memory.nodes.clone(),
            edges: memory.edges.clone(),
            file_extensions: memory.file_extensions.clone(),
            episodes: Vec::new(), // Initialiser avec un vecteur vide d'épisodes
        }
    }

    pub fn to_memory_graph(&self) -> MemoryGraph {
        MemoryGraph {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
            file_extensions: self.file_extensions.clone(),
        }
    }

    pub fn save_bin(&self, path: &str) {
        let file = File::create(path).expect("Could not create bin file");
        bincode::serialize_into(file, self).expect("Could not serialize bin");
    }

    pub fn load_bin(path: &str) -> Self {
        let file = File::open(path).expect("Could not open bin file");
        bincode::deserialize_from(file).expect("Could not deserialize bin")
    }

    pub fn save_ron(&self, path: &str) {
        let file = File::create(path).expect("Could not create RON file");
        ron::ser::to_writer(file, self).expect("Could not serialize RON");
    }

    pub fn load_ron(path: &str) -> Self {
        let file = File::open(path).expect("Could not open RON file");
        ron::de::from_reader(file).expect("Could not deserialize RON")
    }

    pub fn load_default() -> Self {
        Self::load_bin("memory.bin")
            .or_else(|_| Self::load_ron("memory.ron"))
            .unwrap_or_else(|_| Self::new())
    }

    pub fn add_snippet(&mut self, code: String, path: String) {
        let id = Uuid::new_v7();
        let node = MemoryNode {
            id,
            kind: NodeType::SourceSnippet { code, file: path },
            successes: 0,
            failures: 0,
        };
        self.nodes.insert(id, node);
    }

    pub fn pick_random_snippet(&self) -> Option<(String, Uuid)> {
        let mut rng = rand::thread_rng();
        self.nodes.values().filter_map(|node| {
            if let NodeType::SourceSnippet { code, .. } = &node.kind {
                Some((code.clone(), node.id))
            } else {
                None
            }
        }).choose(&mut rng)
    }

    pub fn feedback(
        &mut self,
        id: Uuid,
        mutated_code: &str,
        success: bool,
        mutation_type: &str,
        mutation_reason: &str,
        error_message: Option<String>,
    ) {
        if let Some(node) = self.nodes.get_mut(&id) {
            if success {
                node.successes += 1;
            } else {
                node.failures += 1;
                if let Some(err) = &error_message {
                    node.error_history.push(err.clone());
                }
            }
            node.mutation_count += 1;
        }

        self.add_mutated_node(
            id,
            mutated_code.to_string(),
            mutation_type,
            mutation_reason,
            success,
            error_message,
        );
    }

    /// Charge uniquement les snippets nécessaires pour une opération spécifique
    pub fn load_snippets_by_limit(&self, limit: usize) -> Vec<(String, Uuid)> {
        self.nodes.values()
            .filter_map(|node| {
                if let NodeType::SourceSnippet { code, .. } = &node.kind {
                    Some((code.clone(), node.id))
                } else {
                    None
                }
            })
            .take(limit) // Limite le nombre de snippets chargés
            .collect()
    }

    /// Implémente une pagination pour traiter les données par blocs
    pub fn load_snippets_paginated(&self, page: usize, page_size: usize) -> Vec<(String, Uuid)> {
        self.nodes.values()
            .filter_map(|node| {
                if let NodeType::SourceSnippet { code, .. } = &node.kind {
                    Some((code.clone(), node.id))
                } else {
                    None
                }
            })
            .skip(page * page_size) // Ignore les éléments des pages précédentes
            .take(page_size) // Charge uniquement les éléments de la page actuelle
            .collect()
    }

    /// Ajoute un index pour accéder rapidement aux snippets ou aux noeuds spécifiques
    pub fn index_snippets_by_success(&self) -> Vec<(Uuid, u32)> {
        self.nodes.iter()
            .filter_map(|(id, node)| {
                if let NodeType::SourceSnippet { .. } = node.kind {
                    Some((*id, node.successes))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Sauvegarde une version spécifique de la mémoire
    pub fn save_version(&self, version: &str) {
        let path = format!("memory_{}.bin", version);
        self.save_bin(&path);
    }

    /// Charge une version spécifique de la mémoire
    pub fn load_version(version: &str) -> Self {
        let path = format!("memory_{}.bin", version);
        Self::load_bin(&path).unwrap_or_else(|_| Self::new())
    }

    /// Exporte les données en CSV
    pub fn export_to_csv(&self, path: &str) {
        let mut file = File::create(path).expect("Impossible de créer le fichier CSV");
        writeln!(file, "id,kind,successes,failures,last_error,timestamp").ok();

        for (id, node) in &self.nodes {
            let kind = match &node.kind {
                NodeType::SourceSnippet { .. } => "SourceSnippet",
                NodeType::MutatedSnippet { .. } => "MutatedSnippet",
            };
            let last_error = node.last_compilation_error.clone().unwrap_or_default();
            let timestamp = node.timestamp.unwrap_or(0);
            writeln!(
                file,
                "{},{},{},{},{},{}",
                id, kind, node.successes, node.failures, last_error, timestamp
            )
            .ok();
        }
    }

    /// Ajoute un timestamp et une erreur de compilation à un noeud
    pub fn update_node_metadata(&mut self, id: Uuid, error: Option<String>, timestamp: u64) {
        if let Some(node) = self.nodes.get_mut(&id) {
            node.last_compilation_error = error;
            node.timestamp = Some(timestamp);
        }
    }

    /// Export du graphe en format DOT
    pub fn export_to_dot(&self, path: &str) {
        let mut file = File::create(path).expect("Impossible de créer le fichier DOT");
        writeln!(file, "digraph MemoryGraph {{").ok();

        for (id, node) in &self.nodes {
            let label = match &node.kind {
                NodeType::SourceSnippet { .. } => "SourceSnippet",
                NodeType::MutatedSnippet { mutation_type, .. } => mutation_type,
            };
            let cluster = node.cluster_id.unwrap_or(0);
            writeln!(
                file,
                "    \"{}\" [label=\"{}\", cluster=\"{}\", successes=\"{}\", failures=\"{}\"]",
                id, label, cluster, node.successes, node.failures
            ).ok();
        }

        for (from, to, edge) in &self.edges {
            writeln!(
                file,
                "    \"{}\" -> \"{}\" [label=\"{}\", weight=\"{}\", strength=\"{}\", relation_type=\"{}\"]",
                from, to, edge.description, edge.weight, edge.strength, edge.relation_type
            ).ok();
        }

        writeln!(file, "}}").ok();
    }

    /// Ajoute un épisode à la mémoire
    pub fn add_episode(
        &mut self,
        node_id: Uuid,
        parents: Vec<Uuid>,
        strategy: Strategy,
        context: String,
        code_uuids: Vec<Uuid>,
        log_uuids: Vec<Uuid>,
        success: bool,
        comments: Option<String>,
    ) {
        let episode = MemoryEpisode::new(
            node_id,
            parents,
            strategy,
            context,
            code_uuids,
            log_uuids,
            success,
            comments,
        );
        self.episodes.push(episode);
    }

    /// Sauvegarde les épisodes dans un fichier séparé
    pub fn save_episodes(&self, path: &str) {
        let file = File::create(path).expect("Impossible de créer le fichier des épisodes");
        serde_json::to_writer(file, &self.episodes).expect("Impossible de sérialiser les épisodes");
    }

    /// Charge les épisodes depuis un fichier
    pub fn load_episodes(path: &str) -> Vec<MemoryEpisode> {
        let file = File::open(path).expect("Impossible d'ouvrir le fichier des épisodes");
        serde_json::from_reader(file).expect("Impossible de désérialiser les épisodes")
    }

    /// Sauvegarde les épisodes dans des fichiers partitionnés par date
    pub fn save_partitioned_episodes(&self, base_path: &str) {
        let mut episodes_by_date: HashMap<String, Vec<&MemoryEpisode>> = HashMap::new();

        for episode in &self.episodes {
            let date = chrono::NaiveDateTime::from_timestamp(episode.timestamp as i64, 0)
                .format("%Y-%m-%d")
                .to_string();
            episodes_by_date.entry(date).or_default().push(episode);
        }

        for (date, episodes) in episodes_by_date {
            let path = format!("{}/episodes_{}.json", base_path, date);
            let file = File::create(&path).expect("Impossible de créer le fichier des épisodes partitionnés");
            serde_json::to_writer(file, &episodes).expect("Impossible de sérialiser les épisodes");
        }
    }

    /// Charge les épisodes depuis des fichiers partitionnés
    pub fn load_partitioned_episodes(base_path: &str) -> Vec<MemoryEpisode> {
        let mut all_episodes = Vec::new();
        for entry in std::fs::read_dir(base_path).expect("Impossible de lire le répertoire des épisodes") {
            let entry = entry.expect("Impossible de lire une entrée");
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                let file = File::open(entry.path()).expect("Impossible d'ouvrir le fichier des épisodes");
                let episodes: Vec<MemoryEpisode> = serde_json::from_reader(file).expect("Impossible de désérialiser les épisodes");
                all_episodes.extend(episodes);
            }
        }
        all_episodes
    }

    /// Génère des statistiques sur les épisodes
    pub fn generate_analytics(&self) -> String {
        let mut strategy_stats: HashMap<String, (u32, u32)> = HashMap::new(); // (succès, échecs)

        for episode in &self.episodes {
            let entry = strategy_stats.entry(format!("{:?}", episode.strategy)).or_insert((0, 0));
            if episode.success {
                entry.0 += 1; // Succès
            } else {
                entry.1 += 1; // Échec
            }
        }

        let mut report = String::new();
        report.push_str("Statistiques des stratégies :\n");
        for (strategy, (successes, failures)) in strategy_stats {
            report.push_str(&format!(
                "Stratégie : {} | Succès : {} | Échecs : {}\n",
                strategy, successes, failures
            ));
        }

        report
    }

    /// Sauvegarde un artefact dans un répertoire dédié et retourne son chemin
    pub fn save_artifact(base_path: &str, content: &str, extension: &str) -> String {
        let uuid = Uuid::new_v7().to_string();
        let path = format!("{}/{}.{}", base_path, uuid, extension);
        let mut file = File::create(&path).expect("Impossible de créer le fichier artefact");
        file.write_all(content.as_bytes()).expect("Impossible d'écrire dans le fichier artefact");
        path
    }

    /// Ajoute une image et retourne son chemin
    pub fn add_image(&self, base_path: &str, bytes: Vec<u8>, format: String) -> String {
        let image = MemoryImage::new(bytes, format);
        let path = format!("{}/{}.{}", base_path, image.uuid, image.format);
        image.save(&path).expect("Impossible de sauvegarder l'image");
        path
    }

    /// Ajoute un code source et retourne son chemin
    pub fn add_code(&self, base_path: &str, code: String, language: String) -> String {
        let memory_code = MemoryCode::new(code, language);
        let path = format!("{}/{}.{}", base_path, memory_code.uuid, "rs");
        memory_code.save(&path).expect("Impossible de sauvegarder le code");
        path
    }

    /// Ajoute un log et retourne son chemin
    pub fn add_log(&self, base_path: &str, content: String) -> String {
        let log = MemoryLog::new(content);
        let path = format!("{}/{}.{}", base_path, log.uuid, "log");
        log.save(&path).expect("Impossible de sauvegarder le log");
        path
    }

    /// Ajoute un nœud et le persiste
    pub fn add_node_and_persist(&mut self, node: MemoryNode, path: &str) {
        NodeManager::add_node(self, node);
        self.save_bin(path);
    }

    /// Ajoute une arête et la persiste
    pub fn add_edge_and_persist(
        &mut self,
        from: Uuid,
        to: Uuid,
        description: &str,
        weight: u32,
        strength: f32,
        relation_type: &str,
        is_dependency: bool,
        path: &str,
    ) {
        NodeManager::add_edge(self, from, to, description, weight, strength, relation_type, is_dependency);
        self.save_bin(path);
    }
}
