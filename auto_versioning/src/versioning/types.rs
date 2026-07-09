use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct VersionMeta {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct ChangeSet {
    pub path: String,
    pub version: String,
    pub timestamp: u64,
}
