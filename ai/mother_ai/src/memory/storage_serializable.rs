use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableNode {
    pub id: String,
    pub label: String,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableLink {
    pub source: String,
    pub target: String,
    pub label: Option<String>,
    pub weight: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableGraph {
    pub nodes: Vec<SerializableNode>,
    pub links: Vec<SerializableLink>,
}
