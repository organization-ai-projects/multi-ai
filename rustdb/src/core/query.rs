use std::collections::HashMap;
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub filter: HashMap<String, Value>,
    pub sort: Option<Vec<(String, i32)>>,
    pub limit: Option<u64>,
}

impl Query {
    pub fn new() -> Self {
        Self {
            filter: HashMap::new(),
            sort: None,
            limit: None,
        }
    }

    pub fn with_filter(mut self, key: &str, value: Value) -> Self {
        self.filter.insert(key.to_string(), value);
        self
    }

    pub fn with_sort(mut self, field: &str, direction: i32) -> Self {
        let sort = self.sort.get_or_insert(Vec::new());
        sort.push((field.to_string(), direction));
        self
    }

    pub fn with_limit(mut self, limit: u64) -> Self {
        self.limit = Some(limit);
        self
    }
}
