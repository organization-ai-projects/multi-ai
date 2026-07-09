use std::collections::HashMap;
use std::fs::{File, create_dir_all, remove_file};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::common::error::{MemoryResult, MemoryError};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Node {
    id: String,
    label: String,
    attributes: HashMap<String, String>,
    tags: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Link {
    source: String,
    target: String,
    label: Option<String>,
    weight: Option<f32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializableGraph {
    nodes: Vec<Node>,
    links: Vec<Link>,
}

const MEMORY_FILENAME: &str = "memory";

pub struct GraphMemory {
    nodes: HashMap<String, Node>,
    links: Vec<Link>,
    batch_mode: bool,
    dirty: bool,
}

impl GraphMemory {
    pub fn new(memory_path: String) -> Self {
        // Le chemin est directement fourni par PathManager
        Self {
            nodes: HashMap::new(),
            links: Vec::new(),
            batch_mode: false,
            dirty: false,
        }
    }

    /// Génère tous les chemins nécessaires pour les fichiers mémoire.
    fn get_all_file_paths(base_path: &str) -> HashMap<&'static str, String> {
        let extensions = ["bin", "ron"];
        extensions
            .iter()
            .map(|&ext| (ext, format!("{}/{}.{}", base_path, MEMORY_FILENAME, ext)))
            .collect()
    }

    // ---------------- PERSISTENCE ----------------

    /// Sauvegarde un objet sérialisable dans un fichier avec le format spécifié.
    fn save_to_file<T: serde::Serialize>(path: &str, data: &T, format: &str) -> MemoryResult<()> {
        let file = BufWriter::new(File::create(path)?);
        match format {
            "ron" => ron::ser::to_writer(file, data)
                .map_err(|e| MemoryError::SerializationError(e.to_string()))?,
            "bin" => bincode_next::encode_into_std_write(data, &mut file, bincode_next::config::standard())
                .map_err(|e| MemoryError::SerializationError(e.to_string()))?,
            _ => return Err(MemoryError::InvalidPath(format!("Format inconnu : {}", format))),
        }
        Ok(())
    }

    /// Charge un objet désérialisable depuis un fichier avec le format spécifié.
    fn load_from_file<T: serde::de::DeserializeOwned>(path: &str, format: &str) -> MemoryResult<T> {
        let file = BufReader::new(File::open(path)?);
        match format {
            "ron" => ron::de::from_reader(file)
                .map_err(|e| MemoryError::DeserializationError(e.to_string())),
            "bin" => bincode_next::decode_from_std_read(&mut file, bincode_next::config::standard())
                .map_err(|e| MemoryError::DeserializationError(e.to_string())),
            _ => Err(MemoryError::InvalidPath(format!("Format inconnu : {}", format))),
        }
    }

    pub fn save(&mut self, memory_path: &str) -> MemoryResult<()> {
        create_dir_all(memory_path)?;

        let graph = SerializableGraph {
            nodes: self.nodes.values().cloned().collect(),
            links: self.links.clone(),
        };

        let paths = Self::get_all_file_paths(memory_path);

        Self::save_to_file(&paths["ron"], &graph, "ron")?;
        Self::save_to_file(&paths["bin"], &graph, "bin")?;

        self.dirty = false;
        Ok(())
    }

    pub fn load(&mut self, memory_path: &str) -> MemoryResult<()> {
        let paths = Self::get_all_file_paths(memory_path);

        if Path::new(&paths["bin"]).exists() {
            let graph: SerializableGraph = Self::load_from_file(&paths["bin"], "bin")?;
            return self.restore(graph);
        }

        if Path::new(&paths["ron"]).exists() {
            let graph: SerializableGraph = Self::load_from_file(&paths["ron"], "ron")?;
            return self.restore(graph);
        }

        Err(MemoryError::InvalidPath(format!("Aucun fichier mémoire trouvé dans '{}'", memory_path)))
    }

    pub fn delete(memory_path: &str) -> MemoryResult<()> {
        let paths = Self::get_all_file_paths(memory_path);

        if Path::new(&paths["ron"]).exists() {
            remove_file(&paths["ron"])?;
        }
        if Path::new(&paths["bin"]).exists() {
            remove_file(&paths["bin"])?;
        }

        Ok(())
    }

    fn restore(&mut self, graph: SerializableGraph) -> MemoryResult<()> {
        self.nodes.clear();
        self.links.clear();

        for node in graph.nodes {
            self.nodes.insert(node.id.clone(), node);
        }

        self.links = graph.links;
        Ok(())
    }

    // ---------------- BATCH ----------------

    pub fn begin_batch(&mut self) {
        self.batch_mode = true;
    }

    pub fn flush(&mut self, ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        if self.dirty {
            self.save(ia_name, root)?;
        }
        Ok(())
    }

    fn mark_dirty(&mut self) {
        if self.batch_mode {
            self.dirty = true;
        } else {
            let _ = self.save("default", None); // auto-flush
        }
    }

    // ---------------- MÉMOIRE MÉTIER ----------------

    pub fn create_node(&mut self, id: &str, label: &str) {
        self.mark_dirty();
        self.nodes.insert(id.to_string(), Node {
            id: id.to_string(),
            label: label.to_string(),
            attributes: HashMap::new(),
            tags: Vec::new(),
        });
    }

    pub fn add_attribute(&mut self, id: &str, key: &str, value: &str) {
        self.mark_dirty();
        if let Some(node) = self.nodes.get_mut(id) {
            node.attributes.insert(key.to_string(), value.to_string());
        }
    }

    pub fn add_tag(&mut self, id: &str, tag: &str) {
        self.mark_dirty();
        if let Some(node) = self.nodes.get_mut(id) {
            if !node.tags.contains(&tag.to_string()) {
                node.tags.push(tag.to_string());
            }
        }
    }

    pub fn has_tag(&self, id: &str, tag: &str) -> bool {
        self.nodes.get(id)
            .map(|n| n.tags.contains(&tag.to_string()))
            .unwrap_or(false)
    }

    pub fn add_link(&mut self, from: &str, to: &str, label: Option<String>, weight: Option<f32>) {
        self.mark_dirty();
        if self.nodes.contains_key(from) && self.nodes.contains_key(to) {
            self.links.push(Link {
                source: from.to_string(),
                target: to.to_string(),
                label,
                weight,
            });
        }
    }

    pub fn node_exists(&self, id: &str) -> bool {
        self.nodes.contains_key(id)
    }

    pub fn get_node_label(&self, id: &str) -> Option<String> {
        self.nodes.get(id).map(|n| n.label.clone())
    }

    pub fn get_node_attribute(&self, id: &str, key: &str) -> Option<String> {
        self.nodes.get(id)?.attributes.get(key).cloned()
    }

    pub fn get_all_node_ids(&self) -> Vec<String> {
        self.nodes.keys().cloned().collect()
    }

    pub fn get_links_for_all_nodes(&self) -> Vec<(String, String, Option<String>, Option<f32>)> {
        self.links.iter().map(|l| (
            l.source.clone(),
            l.target.clone(),
            l.label.clone(),
            l.weight,
        )).collect()
    }

    pub fn get_links_for_node(&self, id: &str) -> Vec<(String, String, Option<String>, Option<f32>)> {
        self.links.iter()
            .filter(|l| l.source == id)
            .map(|l| (
                l.source.clone(),
                l.target.clone(),
                l.label.clone(),
                l.weight,
            )).collect()
    }

    /// Sauvegarde un snapshot du graphe avec un nom spécifique.
    pub fn save_snapshot(&self, ia_name: &str, snapshot_name: &str, root: Option<&str>) -> MemoryResult<()> {
        let base_path = Self::build_path(ia_name, root)?;
        create_dir_all(&base_path)?;

        let snapshot_path = format!("{}/snapshots", base_path);
        create_dir_all(&snapshot_path)?;

        let snapshot_file = format!("{}/{}.bin", snapshot_path, snapshot_name);
        let graph = SerializableGraph {
            nodes: self.nodes.values().cloned().collect(),
            links: self.links.clone(),
        };

        let file = BufWriter::new(File::create(&snapshot_file)?);
        bincode_next::encode_into_std_write(&graph, &mut file, bincode_next::config::standard())
            .map_err(|e| MemoryError::SerializationError(e.to_string()))?;

        Ok(())
    }

    /// Charge un snapshot du graphe avec un nom spécifique.
    pub fn load_snapshot(&mut self, ia_name: &str, snapshot_name: &str, root: Option<&str>) -> MemoryResult<()> {
        let base_path = Self::build_path(ia_name, root)?;
        let snapshot_file = format!("{}/snapshots/{}.bin", base_path, snapshot_name);

        if Path::new(&snapshot_file).exists() {
            let reader = BufReader::new(File::open(&snapshot_file)?);
            let graph: SerializableGraph = bincode_next::decode_from_std_read(&mut reader, bincode_next::config::standard())
                .map_err(|e| MemoryError::DeserializationError(e.to_string()))?;
            return self.restore(graph);
        }

        Err(MemoryError::InvalidPath(format!("Snapshot '{}' introuvable pour '{}'", snapshot_name, ia_name)))
    }

    /// Remplace un nœud existant par un nouveau nœud.
    pub fn replace_node(&mut self, id: &str, new_node: Node) -> MemoryResult<()> {
        if self.nodes.contains_key(id) {
            self.mark_dirty();
            self.nodes.insert(id.to_string(), new_node);
            Ok(())
        } else {
            Err(MemoryError::InvalidPath(format!("Nœud '{}' introuvable", id)))
        }
    }

    /// Réinitialise les données en mémoire (nœuds et liens).
    pub fn purge(&mut self) {
        self.nodes.clear();
        self.links.clear();
        self.dirty = false;
        println!("🧹 Mémoire purgée avec succès.");
    }
}
