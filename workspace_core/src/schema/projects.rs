use serde::{Deserialize, Serialize};

// Uniquement la structure attendue, sans les champs techniques
#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct ProjectDocument {
    pub name: String,
    pub status: bool,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub enum ProjectType {
    #[serde(rename = "ai_agent")]
    AiAgent,
    #[serde(rename = "tool")]
    Tool,
}

pub const COLLECTION_NAME: &str = "projects";
