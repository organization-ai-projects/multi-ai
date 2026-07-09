use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct ChangeNode {
    pub path: String,
    pub version: String,
    pub timestamp: u64,
}
