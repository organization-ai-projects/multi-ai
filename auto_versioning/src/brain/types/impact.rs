use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Impact {
    pub level: String,  // "major", "minor", "patch"
    pub confidence: f64,
}
