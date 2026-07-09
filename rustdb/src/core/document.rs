use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct Document<T> {
    pub _id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub data: T,
    pub references: Option<Vec<DbRef>>,
}

impl<T> Document<T> {
    pub fn new(data: T) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Self {
            _id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            data,
            references: None,
        }
    }
}

pub trait DocumentAccess {
    fn get_field_value(&self, field: &str) -> Option<String>;
}

impl<T: Serialize> DocumentAccess for Document<T> {
    fn get_field_value(&self, field: &str) -> Option<String> {
        match serde_json::to_value(&self.data) {
            Ok(Value::Object(map)) => map.get(field).and_then(|v| match v {
                Value::String(s) => Some(s.clone()),
                Value::Number(n) => Some(n.to_string()),
                Value::Bool(b) => Some(b.to_string()),
                Value::Array(a) => Some(serde_json::to_string(a).unwrap_or_default()),
                _ => None,
            }),
            _ => None,
        }
    }
}
