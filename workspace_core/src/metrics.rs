use metrics::{counter, gauge, histogram};
use std::time::Instant;

pub struct DatabaseMetrics {
    query_timer: Instant,
}

impl DatabaseMetrics {
    pub fn track_query(&mut self, collection: &str, operation: &str) {
        let duration = self.query_timer.elapsed();
        histogram!("database.query.duration", duration, "collection" => collection, "operation" => operation);
        counter!("database.query.count", 1, "collection" => collection, "operation" => operation);
    }
}
