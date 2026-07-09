use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct SerializableNode {
    pub id: String,
    pub label: String,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct SerializableLink {
    pub source: String,
    pub target: String,
    pub label: Option<String>,
    pub weight: Option<f32>,
}

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct SerializableGraph {
    pub nodes: Vec<SerializableNode>,
    pub links: Vec<SerializableLink>,
}
