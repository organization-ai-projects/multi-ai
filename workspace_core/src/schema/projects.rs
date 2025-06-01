use serde::{Deserialize, Serialize};

// Uniquement la structure attendue, sans les champs techniques
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectDocument {
    pub name: String,
    pub status: bool,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ProjectType {
    #[serde(rename = "ai_agent")]
    AiAgent,
    #[serde(rename = "tool")]
    Tool,
}
