use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default)]
pub struct Memory {
    pub data: Vec<f64>,
    pub last_reward: f64,
    pub use_neuron: bool, // Peut être muté par évolution
                          // Tu peux rajouter d'autres champs "évolutifs"
}

pub fn load() -> std::io::Result<Memory> {
    match fs::read("memory.bin") {
        Ok(data) => Ok(bincode::deserialize(&data).unwrap_or_default()),
        Err(_) => Ok(Memory::default()),
    }
}

pub fn save(memory: &Memory) {
    if let Ok(data) = bincode::serialize(memory) {
        let _ = fs::write("memory.bin", data);
    }
    if let Ok(state) = ron::to_string(memory) {
        let _ = fs::write("memory.ron", state);
    }
}
