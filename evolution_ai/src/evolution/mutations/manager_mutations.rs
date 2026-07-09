use super::strategies::StrategiesManager;

pub struct MutationManager {
    strategies_manager: StrategiesManager,
}

impl MutationManager {
    pub fn new() -> Self {
        Self {
            strategies_manager: StrategiesManager::new(0.2),
        }
    }

    pub fn mutate_code(&mut self, code: &str, strategy: &str) -> String {
        self.strategies_manager.mutate(code, strategy, 3).code
    }
}
