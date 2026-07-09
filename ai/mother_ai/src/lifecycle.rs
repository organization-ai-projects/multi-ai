use crate::memory::graph_memory::GraphMemory;
use crate::memory::path_manager::PathManager;
use crate::training::trainer;
use std::error::Error;

/// Gère le cycle de vie complet de l'IA.
pub struct LifecycleManager {
    pub ia_name: String,
    pub memory: GraphMemory,
}

impl LifecycleManager {
    /// Initialise le cycle de vie avec mémoire graphique.
    pub fn new(ia_name: String) -> Result<Self, Box<dyn Error>> {
        let path_manager = PathManager::load_from_bin("shared_center/ia_list.bin")?;
        let memory_path = path_manager
            .get_memory_path(&ia_name, None)
            .ok_or("Chemin mémoire introuvable pour cette IA")?;
        let memory = GraphMemory::new(memory_path.clone());

        Ok(Self { ia_name, memory })
    }

    /// Exécute le cycle de vie complet de l'IA.
    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // Exemple d'entraînement
        let timestamp = format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs());
        let node_id = format!("training_{}", timestamp);

        let (training_result, training_kind) = match trainer::execute_training("Données d'entraînement") {
            Ok(kind) => ("succès", kind),
            Err(err) => {
                println!("⚠️ Erreur d'entraînement : {}", err);
                ("échec", "unknown".to_string())
            }
        };

        // Manipule la mémoire graphique
        self.memory.create_node(&node_id, &format!("Entraînement {}", training_kind));
        self.memory.add_attribute(&node_id, "résultat", training_result);
        self.memory.add_tag(&node_id, "training");
        self.memory.add_tag(&node_id, &training_kind);

        // Sauvegarde en utilisant le chemin mémoire
        self.memory.flush()?;
        self.memory.purge(); // Réintroduction confirmée

        println!("✅ IA {} entraînée avec succès", self.ia_name);
        Ok(())
    }
}
