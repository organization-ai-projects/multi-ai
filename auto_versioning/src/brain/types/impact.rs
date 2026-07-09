use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Impact {
    pub level: String,  // "major", "minor", "patch"
    pub confidence: f64,
}
