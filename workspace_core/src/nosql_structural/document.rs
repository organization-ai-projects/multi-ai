use crate::schema::ProjectDocument;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use super::references::DbRef;
use std::convert::From;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Document<T> {
    pub _id: Uuid,
    pub created_at: i64,
    pub updated_at: i64,
    pub data: T,
    pub references: Option<Vec<DbRef>>,
}

pub trait DocumentAccess {
    fn get_field_value(&self, field: &str) -> Option<String>;
}

impl<T: Serialize> DocumentAccess for Document<T> {
    fn get_field_value(&self, field: &str) -> Option<String> {
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

impl<T> From<T> for Document<T> {
    fn from(data: T) -> Self {
        Self {
            _id: Uuid::now_v7(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            updated_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
            data,
            references: None,
        }
    }
}
