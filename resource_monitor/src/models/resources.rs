pub struct ResourceThresholds {
    pub max_cpu_usage: f32,           // 0.0 - 100.0
    pub max_memory_usage: f32,        // 0.0 - 100.0
    pub sampling_interval_ms: u64,    // Intervalle d'échantillonnage
    pub cooldown_period_ms: u64,      // Période de refroidissement après une action
}

impl Default for ResourceThresholds {
    fn default() -> Self {
        Self {
            max_cpu_usage: 80.0,
            max_memory_usage: 80.0,
            sampling_interval_ms: 1000,
            cooldown_period_ms: 5000,
        }
    }
}

impl Clone for ResourceThresholds {
    fn clone(&self) -> Self {
        Self {
            max_cpu_usage: self.max_cpu_usage,
            max_memory_usage: self.max_memory_usage,
            sampling_interval_ms: self.sampling_interval_ms,
            cooldown_period_ms: self.cooldown_period_ms,
        }
    }
}
