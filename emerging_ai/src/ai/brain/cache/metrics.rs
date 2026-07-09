use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

pub struct CacheMetrics {
    hits_by_type: HashMap<String, AtomicU64>,
    misses_by_type: HashMap<String, AtomicU64>,
    evictions_by_type: HashMap<String, AtomicU64>,
    latencies: Vec<Duration>,
}

impl CacheMetrics {
    pub fn record_operation(&mut self, entity_type: &str, operation: &str, duration: Duration) {
        if let Some(counter) = match operation {
            "hit" => self.hits_by_type.get(entity_type),
            "miss" => self.misses_by_type.get(entity_type),
            "evict" => self.evictions_by_type.get(entity_type),
            _ => None,
        } {
            counter.fetch_add(1, Ordering::SeqCst);
        }
        self.latencies.push(duration);
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        // Stats par type d'entité
        for (entity_type, hits) in &self.hits_by_type {
            let hits = hits.load(Ordering::SeqCst);
            let misses = self.misses_by_type[entity_type].load(Ordering::SeqCst);
            let evictions = self.evictions_by_type[entity_type].load(Ordering::SeqCst);
            let total = hits + misses;

            report.push_str(&format!(
                "{}: hits={}, misses={}, hit_ratio={:.2}%, evictions={}\n",
                entity_type,
                hits,
                misses,
                (hits as f64 / total as f64) * 100.0,
                evictions
            ));
        }

        // Latences
        if !self.latencies.is_empty() {
            let avg_latency = self.latencies.iter().sum::<Duration>() / self.latencies.len() as u32;
            report.push_str(&format!("Average latency: {:?}\n", avg_latency));
        }

        report
    }
}
