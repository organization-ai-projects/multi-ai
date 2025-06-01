use crate::core::{Document, Query, error::Result};
use serde::de::DeserializeOwned;

pub trait DocumentReader<T: DeserializeOwned> {
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Document<T>>;
    async fn find_many(&self, query: Query) -> Result<Vec<Document<T>>>;
    async fn find_one(&self, query: Query) -> Result<Option<Document<T>>>;
}
