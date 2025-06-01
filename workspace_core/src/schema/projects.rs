use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Collection Projects - Représente tous les projets du workspace
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectsCollection {
    pub _id: Uuid,
    pub name: String,        // Toujours "projects"
    pub project_type: ProjectType,
    pub documents: Vec<ProjectDocument>,
}

/// Type de projet
#[derive(Debug, Deserialize, Serialize)]
pub enum ProjectType {
    #[serde(rename = "ai_agent")]
    AiAgent,
    #[serde(rename = "tool")]
    Tool,
}

/// Document de projet
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectDocument {
    pub _id: Uuid,
    pub name: String,
    pub status: bool,
    pub tags: Option<Vec<String>>,
}
