use crate::Query;

use super::document::{Document, DocumentAccess};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct Collection<T> {
    pub _id: Uuid,
    pub name: String,
    pub documents: Vec<Document<T>>,
    pub indexes: HashMap<String, Index>,
}

impl<T> Collection<T> {
    pub fn new(name: &str) -> Self {
        Self {
            _id: Uuid::new_v4(),
            name: name.to_string(),
            documents: Vec::new(),
            indexes: HashMap::new(),
        }
    }

    pub fn find(&self, query: Query) -> Vec<Document<T>> {
        let mut results = self
            .documents
            .iter()
            .filter(|doc| {
                query.filter.iter().all(|(key, value)| {
                    doc.get_field_value(key)
                        .map(|field_value| field_value == value)
                        .unwrap_or(false)
                })
            })
            .cloned()
            .collect::<Vec<_>>();

        // Appliquer le tri
        if let Some(sort_fields) = &query.sort {
            results.sort_by(|a, b| {
                for (field, order) in sort_fields {
                    match (a.get_field_value(field), b.get_field_value(field)) {
                        (Some(va), Some(vb)) => {
                            let cmp = va.cmp(&vb);
                            if cmp != std::cmp::Ordering::Equal {
                                return if *order > 0 { cmp } else { cmp.reverse() };
                            }
                        }
                        _ => continue,
                    }
                }
                std::cmp::Ordering::Equal
            });
        }

        // Appliquer la limite
        if let Some(limit) = query.limit {
            results.truncate(limit as usize);
        }

        results
    }
}
