mod logic;
mod memory;
mod neuron;

use crate::memory::Memory;
use std::{fs, thread};

fn main() {
    // Charger la mémoire (persistante, ou par défaut)
    let mut memory = memory::load().unwrap_or_default();

    loop {
        // 1. Lire l'entrée (input.bin)
        let input = match fs::read("input.bin") {
            Ok(data) => bincode_next::decode_from_slice(&data, bincode_next::config::standard()).map(|(v, _)| v).unwrap_or_default(),
            Err(_) => Vec::<f64>::new(),
        };

        // 2. Décider quel "cerveau" utiliser (logique, NN, autre...)
        let output = if memory.use_neuron {
            neuron::process(&input, &mut memory)
        } else {
            logic::process(&input, &mut memory)
        };

        // 3. Écrire la sortie
        if let Ok(data) = bincode_next::encode_to_vec(&output, bincode_next::config::standard()) {
            let _ = fs::write("output.bin", data);
        }

        // 4. Sauver la mémoire (binaire et debug .ron)
        memory::save(&memory);

        thread::sleep(std::time::Duration::from_millis(100));
    }
}
