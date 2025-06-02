use serde::{Deserialize, Serialize};
use chrono;

#[derive(Deserialize, Serialize)]  // Ajout de Serialize
pub struct IaList {
    pub nature: Vec<String>,
    pub scientist: Vec<String>,
    pub life_form: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct IaMetadata {
    pub id: String,
    pub kind: String,
}

#[derive(Serialize, Deserialize)]
pub struct GlobalMetadata {
    pub version: u32,
    pub last_run: String,
    pub total_runs: u32,
}

impl Default for GlobalMetadata {
    fn default() -> Self {
        Self {
            version: 1,
            last_run: chrono::Local::now().to_rfc3339(),
            total_runs: 0,
        }
    }
}
