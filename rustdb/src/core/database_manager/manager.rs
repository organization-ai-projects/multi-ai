use super::{
    persistence::StorageManager,
    transactions::TransactionManager,
    indexing::IndexManager,
    security::SecurityManager,
    config::DatabaseConfig,
    health::DatabaseHealth
};
use crate::core::{Collection, Document, Query, error::Result};

pub struct DatabaseManager {
    storage: StorageManager,
    transactions: TransactionManager,
    indexes: IndexManager,
    security: SecurityManager,
}

impl DatabaseManager {
    pub fn new(config: DatabaseConfig) -> Self {
        Self {
            storage: StorageManager::new(config.data_path),
            transactions: TransactionManager::new(),
            indexes: IndexManager::new(),
            security: SecurityManager::new(config.security),
        }
    }

    pub async fn create_collection(&self, name: &str, schema: Option<Schema>) -> Result<()>;
    pub async fn backup(&self, strategy: BackupStrategy) -> Result<()>;
    pub async fn health_check(&self) -> DatabaseHealth;
}
