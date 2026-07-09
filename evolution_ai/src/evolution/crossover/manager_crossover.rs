use super::strategy::CrossoverStrategy;
use rand::Rng;

pub struct CrossoverManager {
    strategy: CrossoverStrategy,
}

impl CrossoverManager {
    pub fn new(crossover_rate: f64) -> Self {
        Self {
            strategy: CrossoverStrategy::new(crossover_rate),
        }
    }

    pub fn try_crossover(&self, code1: &str, code2: &str) -> Option<String> {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(self.strategy.crossover_rate) {
            Some(
                self.strategy
                    .crossover(&[code1.to_string(), code2.to_string()], &mut rng),
            )
        } else {
            None
        }
    }
}
