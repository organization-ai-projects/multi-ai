use crate::schema::ProjectDocument;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use super::references::DbRef;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Document<T> {
    pub _id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub data: T,
    pub references: Option<Vec<DbRef>>,
}

impl<T: Serialize> Document<T> {
    pub fn get_field_value(&self, field: &str) -> Option<String> {
        match serde_json::to_value(&self.data) {
            Ok(Value::Object(map)) => {
                map.get(field).and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    Value::Number(n) => Some(n.to_string()),
                    Value::Bool(b) => Some(b.to_string()),
                    Value::Array(a) => Some(serde_json::to_string(a).unwrap_or_default()),
                    _ => None,
                })
            },
            _ => None
        }
    }
}
