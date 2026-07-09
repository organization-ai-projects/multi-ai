use std::fs::File;
use std::io::{BufReader, BufWriter};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use ron;
use bincode;

#[derive(serde::Deserialize, Debug)]
pub struct FormatConfig {
    pub name: String,
    pub extension: String,
}

pub struct FormatManager;

impl FormatManager {
    /// Charge les configurations des formats depuis un fichier `.ron`.
    pub fn load_from_file(file_path: &str) -> HashMap<String, FormatConfig> {
        let file = File::open(file_path).expect("Impossible d'ouvrir le fichier de configuration des formats");
        let reader = BufReader::new(file);
        ron::de::from_reader(reader).expect("Erreur lors du chargement des configurations des formats")
    }

    /// Retourne les sérialiseurs et désérialiseurs disponibles pour chaque format.
    pub fn get_handlers<T: Serialize, U: DeserializeOwned>() -> HashMap<&'static str, (fn(BufWriter<File>, &T) -> Result<(), String>, fn(BufReader<File>) -> Result<U, String>)> {
        HashMap::from([
            (
                "ron",
                (
                    |f, d| ron::ser::to_writer(f, d).map_err(|e| e.to_string()),
                    |f| ron::de::from_reader(f).map_err(|e| e.to_string()),
                ),
            ),
            (
                "bin",
                (
                    |f, d| bincode::serialize_into(f, d).map_err(|e| e.to_string()),
                    |f| bincode::deserialize_from(f).map_err(|e| e.to_string()),
                ),
            ),
        ])
    }
}
