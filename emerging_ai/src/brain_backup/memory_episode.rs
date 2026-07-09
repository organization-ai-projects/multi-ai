use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::capacities::mutate_ast::Strategy;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemoryEpisode {
    pub uuid: Uuid,           // UUID unique de l'épisode
    pub node_id: Uuid,        // Noeud principal concerné
    pub parents: Vec<Uuid>,   // Chemin parcouru (lignée complète)
    pub strategy: Strategy,   // Stratégie appliquée
    pub context: String,      // Description ou contexte de la tentative
    pub code_uuids: Vec<Uuid>, // Références aux artefacts de code
    pub log_uuids: Vec<Uuid>,  // Références aux logs ou autres artefacts
    pub timestamp: u64,       // Timestamp de l'épisode
    pub success: bool,        // Succès ou échec de l'expérience
    pub comments: Option<String>, // Observations ou remarques
}

impl MemoryEpisode {
    /// Crée un nouvel épisode
    pub fn new(
        node_id: Uuid,
        parents: Vec<Uuid>,
        strategy: Strategy,
        context: String,
        code_uuids: Vec<Uuid>,
        log_uuids: Vec<Uuid>,
        success: bool,
        comments: Option<String>,
    ) -> Self {
        MemoryEpisode {
            uuid: Uuid::new_v7(),
            node_id,
            parents,
            strategy,
            context,
            code_uuids,
            log_uuids,
            timestamp: chrono::Utc::now().timestamp() as u64,
            success,
            comments,
        }
    }
}
