use std::collections::HashMap;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use super::Document;

pub trait Searchable {
    fn matches(&self, criteria: &HashMap<String, String>) -> bool;
}

impl<T: Serialize> Searchable for Document<T> {
    fn matches(&self, criteria: &HashMap<String, String>) -> bool {
        match serde_json::to_value(&self.data) {
            Ok(Value::Object(obj)) => {
                criteria.iter().all(|(key, value)| {
                    obj.get(key)
                        .and_then(|v| v.as_str())
                        .map(|v| v == value)
                        .unwrap_or(false)
                })
            }
            _ => false
        }
    }
}

pub struct SearchQuery {
    pub filters: HashMap<String, String>,
    pub sort: Option<Vec<(String, SortOrder)>>,
    pub limit: Option<usize>,
    pub skip: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum SortOrder {
    Ascending,
    Descending,
}
