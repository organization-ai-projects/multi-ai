mod memory;
mod observation;
mod laboratory;
mod transgene;
mod mutation;
mod taxonomy;

use memory::ExperimentMemory;
use observation::Observer;
use laboratory::Laboratory;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Config {
    id: String,
    kind: String, 
    observation_interval_secs: u64,
    memory_path: PathBuf,
    laboratory_path: PathBuf
}

fn load_config() -> Config {
    ron::from_str(&std::fs::read_to_string("metadata.ron").unwrap()).unwrap()
}

fn main() {
    let config = load_config();
    let mut memory = ExperimentMemory::new_from_file(&config.memory_path);
    let mut observer = Observer::new("../nature/environment");
    let mut lab = Laboratory::new(&config.laboratory_path);

    println!("Scientist [{}] démarre...", config.id);
    
    loop {
        // Observer et apprendre
        observer.scan_environment(&mut memory);
        
        // Sauvegarder la mémoire
        memory.save();

        std::thread::sleep(std::time::Duration::from_secs(
            config.observation_interval_secs
        ));
    }
}
