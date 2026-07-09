use std::sync::Arc;
use std::time::Instant;

pub struct DatabaseMetrics {
    query_timer: Arc<Instant>,
}

impl DatabaseMetrics {
    pub fn new() -> Self {
        Self {
            query_timer: Arc::new(Instant::now()),
        }
    }

    pub fn track_query(&self, collection: &str, operation: &str, duration: std::time::Duration) {
        histogram!("database.query.duration", duration, "collection" => collection.to_string(), "operation" => operation.to_string());
        counter!("database.query.count", 1, "collection" => collection.to_string(), "operation" => operation.to_string());
    }
}
