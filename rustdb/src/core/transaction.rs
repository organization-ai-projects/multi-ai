use std::collections::HashMap;
use super::{Document, Collection, error::Result};
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct Transaction {
    id: uuid::Uuid,
    changes: HashMap<String, Vec<Operation>>,
    collections: Arc<RwLock<HashMap<String, Collection<Document>>>>,
}

#[derive(Debug)]
enum Operation {
    Insert(Document<Document>),
    Update { id: uuid::Uuid, doc: Document<Document> },
    Delete(uuid::Uuid),
}

impl Transaction {
    pub async fn commit(self) -> Result<()> {
        let mut collections = self.collections.write().await;
        
        for (coll_name, ops) in self.changes {
            if let Some(collection) = collections.get_mut(&coll_name) {
                for op in ops {
                    match op {
                        Operation::Insert(doc) => collection.insert(doc)?,
                        Operation::Update { id, doc } => collection.update_by_id(id, doc)?,
                        Operation::Delete(id) => collection.delete_by_id(id)?,
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn rollback(self) {
        // Log rollback
        println!("Rolling back transaction {}", self.id);
    }
}
