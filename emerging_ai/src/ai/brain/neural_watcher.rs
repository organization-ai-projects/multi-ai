use std::sync::Arc;
use tokio::sync::broadcast;

pub struct NeuralWatcher {
    neural_network: Arc<NeuralNetwork>,
    memory_center: Arc<MemoryCenter>,
}

impl NeuralWatcher {
    pub fn new() -> Self {
        Self {
            neural_network: Arc::new(NeuralNetwork::new()),
            memory_center: Arc::new(MemoryCenter::new()),
        }
    }

    // Point unique de capture de TOUTES les données
    pub fn observe<T: Observation>(&mut self, data: T) {
        // 1. Capture de la donnée brute
        let raw_signal = data.into_neural_signal();

        // 2. Traitement neuronal automatique
        if let Some(pattern) = self.neural_network.process_signal(raw_signal) {
            // 3. Stockage en mémoire si pertinent
            self.memory_center.store_if_relevant(pattern);
        }
    }
}

// Les signaux neuronaux sont des impulsions brutes
pub struct NeuralSignal {
    intensity: f32,
    pattern: Vec<u8>,
    timestamp: u64,
}
