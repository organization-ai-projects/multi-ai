use std::collections::HashMap;
use super::{Document, error::Result};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    pub name: String,
    pub field_path: String,
    pub unique: bool,
    pub values: HashMap<String, Vec<uuid::Uuid>>,
}

impl Index {
    pub fn new(name: &str, field_path: &str, unique: bool) -> Self {
        Self {
            name: name.to_string(),
            field_path: field_path.to_string(),
            unique,
            values: HashMap::new(),
        }
    }

    pub fn add_document<T: Serialize>(&mut self, doc: &Document<T>) -> Result<()> {
        let value = doc.get_field_value(&self.field_path)?;
        
        if self.unique {
            if self.values.values().any(|ids| !ids.is_empty()) {
                return Err(Error::UniqueConstraintViolation(self.field_path.clone()));
            }
        }

        self.values.entry(value)
            .or_insert_with(Vec::new)
            .push(doc._id);
        
        Ok(())
    }
}
