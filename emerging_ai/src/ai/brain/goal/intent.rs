use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum GoalType {
    MaximizeScore,
    ExplorePatterns,
    OptimizeCode,
    LearnNewPattern,
    Custom(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct Intent {
    pub uuid: Uuid,
    pub goal_type: GoalType,
    pub priority: u8,
    pub progress: f32, // 0.0 - 1.0
    pub success_metric: Option<String>,
    pub deadline: Option<u64>, // timestamp
    pub dependencies: Vec<Uuid>,
}
