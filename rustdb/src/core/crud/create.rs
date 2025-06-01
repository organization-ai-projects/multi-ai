use crate::core::{Document, error::Result};
use serde::Serialize;

pub trait DocumentCreator<T: Serialize> {
    async fn create_document(&self, document: T) -> Result<Document<T>>;
    async fn create_many(&self, documents: Vec<T>) -> Result<Vec<Document<T>>>;
}
