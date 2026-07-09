use metrics::{counter, gauge, histogram};
use std::time::Instant;

pub struct DatabaseMetrics {
    query_timer: Instant,
}

impl DatabaseMetrics {
    pub fn track_query(&mut self, collection: String, operation: String) {
        let duration = self.query_timer.elapsed();
        histogram!(
            "database.query.duration",
            duration,
            "collection" => collection.clone(),
            "operation" => operation.clone()
        );
        counter!(
            "database.query.count",
            1,
            "collection" => collection,
            "operation" => operation
        );
    }
}
