use serde::{Serialize, Deserialize};
use uuid::Uuid;

//contient la mémoire épisodique de l'IA
//utilise les artefacts pour retenir des patterns

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct MemoryEpisode {
    pub uuid: Uuid,           // UUID unique de l'épisode
    pub node_id: Uuid,        // Noeud principal concerné
    pub parents: Vec<Uuid>,   // Chemin parcouru (lignée complète)
    pub strategy: String,     // Stratégie appliquée
    pub context: String,      // Description ou contexte de la tentative
    pub artifact_uuids: Vec<Uuid>, // Références aux artefacts (logs, images, code)
    pub timestamp: u64,       // Timestamp de l'épisode
    pub success: bool,        // Succès ou échec de l'expérience
    pub comments: Option<String>, // Observations ou remarques
}

impl MemoryEpisode {
    pub fn new(
        uuid: Uuid,
        node_id: Uuid,
        parents: Vec<Uuid>,
        strategy: String,
        context: String,
        artifact_uuids: Vec<Uuid>,
        timestamp: u64,
        success: bool,
        comments: Option<String>,
    ) -> Self {
        Self {
            uuid,
            node_id,
            parents,
            strategy,
            context,
            artifact_uuids,
            timestamp,
            success,
            comments,
        }
    }

    pub fn add_parent(&mut self, parent_uuid: Uuid) {
        self.parents.push(parent_uuid);
    }

    pub fn add_artifact_uuid(&mut self, artifact_uuid: Uuid) {
        self.artifact_uuids.push(artifact_uuid);
    }
}