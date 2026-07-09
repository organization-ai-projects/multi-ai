use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default, bincode_next::Encode, bincode_next::Decode)]
pub struct Memory {
    pub data: Vec<f64>,
    pub last_reward: f64,
    pub use_neuron: bool, // Peut être muté par évolution
                          // Tu peux rajouter d'autres champs "évolutifs"
}

pub fn load() -> std::io::Result<Memory> {
    match fs::read("memory.bin") {
        Ok(data) => Ok(bincode_next::decode_from_slice(&data, bincode_next::config::standard()).map(|(v, _)| v).unwrap_or_default()),
        Err(_) => Ok(Memory::default()),
    }
}

pub fn save(memory: &Memory) {
    if let Ok(data) = bincode_next::encode_to_vec(memory, bincode_next::config::standard()) {
        let _ = fs::write("memory.bin", data);
    }
    if let Ok(state) = ron::to_string(memory) {
        let _ = fs::write("memory.ron", state);
    }
}
