use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, bincode_next::Encode, bincode_next::Decode)]
pub struct Needs {
    pub energy: f64,
    pub water: f64,
    pub food: f64,
}

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Resource {
    pub name: String,
    pub amount: f64,
}
