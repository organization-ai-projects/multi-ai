use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionMeta {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeSet {
    pub path: String,
    pub version: String,
    pub timestamp: u64,
}
