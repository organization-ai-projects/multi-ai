use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Needs {
    pub energy: f64,
    pub water: f64,
    pub food: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    pub name: String,
    pub amount: f64,
}
