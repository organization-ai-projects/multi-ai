//gère uniquement les chemins à partir du fichier ia_list.bin
use std::fs::File;
use std::io::{self, BufReader};
use serde::Deserialize;
use bincode;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct IaConfig {
    pub(crate) name: String,
    pub(crate) memory_path: String,
    pub(crate) sandbox_path: String,
    pub(crate) manifest_path: String,
    pub(crate) args: Vec<String>,
}

pub(crate) struct PathManager {
    pub(crate) configs: Vec<IaConfig>,
}

impl PathManager {
    /// Crée une nouvelle instance de PathManager en chargeant les configurations depuis le fichier par défaut
    pub fn new() -> Self {
        match Self::load_from_bin("shared_center/ia_list.bin") {
            Ok(path_manager) => path_manager,
            Err(e) => {
                eprintln!("Erreur lors du chargement des configurations IA: {}", e);
                // Retourner une instance par défaut avec une liste vide si la lecture échoue
                Self { configs: Vec::new() }
            }
        }
    }

    /// Charge les configurations d'IA à partir du fichier binaire ia_list.bin
    /// Ce fichier existe déjà et contient les informations des chemins pour chaque IA
    pub(crate) fn load_from_bin(file_path: &str) -> io::Result<Self> {
        // fichier_path est généralement "shared_center/ia_list.bin"
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let configs: Vec<IaConfig> = bincode::deserialize_from(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(Self { configs })
    }

    pub(crate) fn get_config(&self, ia_name: &str) -> Option<IaConfig> {
        self.configs.iter().find(|c| c.name == ia_name).cloned()
    }

    /// Obtient le chemin de base pour la mémoire d'une IA
    /// Si root est spécifié, il est préfixé au chemin
    pub(crate) fn get_memory_path(&self, ia_name: &str, root: Option<&str>) -> Option<String> {
        self.get_config(ia_name).map(|c| {
            if let Some(prefix) = root.filter(|r| !r.is_empty()) {
                format!("{}/{}", prefix, c.memory_path)
            } else {
                c.memory_path
            }
        })
    }

    pub(crate) fn get_sandbox_path(&self, ia_name: &str) -> Option<String> {
        self.get_config(ia_name).map(|c| c.sandbox_path)
    }

    pub(crate) fn get_manifest_path(&self, ia_name: &str) -> Option<String> {
        self.get_config(ia_name).map(|c| c.manifest_path)
    }
}
