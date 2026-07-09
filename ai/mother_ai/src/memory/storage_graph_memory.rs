use std::fs::{File, create_dir_all, remove_file};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use crate::common::error::{MemoryError, MemoryResult};
use super::{
    manager_graph_memory::GraphMemoryManager,
    path_manager::PathManager,
    storage_serializable::{SerializableGraph, SerializableNode, SerializableLink},
};

const MEMORY_FILENAME: &str = "memory";

pub struct StorageGraphMemory<'a> {
    manager: &'a mut GraphMemoryManager,
}

impl<'a> StorageGraphMemory<'a> {
    pub fn new(manager: &'a mut GraphMemoryManager) -> Self {
        Self { manager }
    }

    // ---------- API publique ----------

    pub fn save(&self, ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        let base_path = PathManager::get_memory_path(ia_name, root)?;
        create_dir_all(&base_path)?;

        let graph = self.build_serializable();

        let ron_file = BufWriter::new(File::create(format!("{}/{}.ron", base_path, MEMORY_FILENAME))?);
        ron::ser::to_writer(ron_file, &graph)?;

        let bin_file = BufWriter::new(File::create(format!("{}/{}.bin", base_path, MEMORY_FILENAME))?);
        bincode_next::encode_into_std_write(&graph, &mut bin_file, bincode_next::config::standard())?;

        Ok(())
    }

    pub fn load(&mut self, ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        let base_path = PathManager::get_memory_path(ia_name, root)?;

        // bincode prioritaire
        let bin_path = format!("{}/{}.bin", base_path, MEMORY_FILENAME);
        if Path::new(&bin_path).exists() {
            let reader = BufReader::new(File::open(&bin_path)?);
            let graph: SerializableGraph = bincode_next::decode_from_std_read(&mut reader, bincode_next::config::standard())?;
            self.restore_from_serializable(graph);
            return Ok(());
        }

        // fallback RON
        let ron_path = format!("{}/{}.ron", base_path, MEMORY_FILENAME);
        if Path::new(&ron_path).exists() {
            let reader = BufReader::new(File::open(&ron_path)?);
            let graph: SerializableGraph = ron::de::from_reader(reader)?;
            self.restore_from_serializable(graph);
            return Ok(());
        }

        Err(MemoryError::InvalidPath(format!("Aucun fichier mémoire trouvé pour '{}'", ia_name)))
    }

    pub fn delete(&self, ia_name: &str, root: Option<&str>) -> MemoryResult<()> {
        let base_path = PathManager::get_memory_path(ia_name, root)?;

        for ext in ["ron", "bin"] {
            let path = format!("{}/{}.{}", base_path, MEMORY_FILENAME, ext);
            if Path::new(&path).exists() {
                remove_file(path)?;
            }
        }
        Ok(())
    }

    // ---------- Helpers privés ----------

    fn build_serializable(&self) -> SerializableGraph {
        let node_ids = self.manager.get_all_node_ids();

        let nodes: Vec<SerializableNode> = node_ids
            .iter()
            .map(|id| SerializableNode {
                id: id.clone(),
                label: self.manager.get_node_label(id).unwrap_or_default(),
                attributes: self.manager.get_node_attributes(id), // simple getter, PAS de sérialisation
            })
            .collect();

        let links: Vec<SerializableLink> = self
            .manager
            .get_links_for_all_nodes()
            .into_iter()
            .map(|(source, target, label, weight)| SerializableLink {
                source,
                target,
                label,
                weight,
            })
            .collect();

        SerializableGraph { nodes, links }
    }

    fn restore_from_serializable(&mut self, graph: SerializableGraph) {
        for node in graph.nodes {
            self.manager.create_and_add_node(&node.id, &node.label);
            if let Some(attrs) = node.attributes {
                for (k, v) in attrs {
                    self.manager.create_node_add_attribute(&node.id, &k, &v);
                }
                self.manager.create_node_finish(&node.id);
            }
        }

        for link in graph.links {
            self.manager
                .create_and_add_link(&link.source, &link.target, link.label, link.weight);
        }
    }
}
