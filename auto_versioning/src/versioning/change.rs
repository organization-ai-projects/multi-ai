use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeNode {
    pub path: String,
    pub version: String,
    pub timestamp: u64,
}
