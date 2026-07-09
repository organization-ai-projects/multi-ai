use serde::{Serialize, Deserialize};
use crate::graph::Impact;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VersionSnapshot {
    pub id: String,
    pub timestamp: i64,
    pub path: String,
    pub impact: Impact,
    pub hash: String,
    pub files_changed: Vec<String>, // Ajout du champ pour les fichiers modifiés
}
