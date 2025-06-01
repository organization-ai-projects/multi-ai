use super::{state::TransactionState, log::TransactionLog};
use uuid::Uuid;
use tokio::sync::RwLock;
use std::sync::Arc;
use std::collections::HashMap;
use super::super::error::Result;

pub struct TransactionCoordinator {
    active_transactions: Arc<RwLock<HashMap<Uuid, TransactionState>>>,
    log: TransactionLog,
}

impl TransactionCoordinator {
    pub fn new(log_path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            active_transactions: Arc::new(RwLock::new(HashMap::new())),
            log: TransactionLog::new(log_path),
        }
    }

    pub async fn begin_transaction(&self) -> Result<Uuid> {
        let tx_id = Uuid::new_v4();
        self.log.write_begin(tx_id).await?;
        
        let mut transactions = self.active_transactions.write().await;
        transactions.insert(tx_id, TransactionState::Active);
        
        Ok(tx_id)
    }

    pub async fn commit(&self, tx_id: Uuid) -> Result<()> {
        let mut transactions = self.active_transactions.write().await;
        if let Some(state) = transactions.get(&tx_id) {
            if *state == TransactionState::Active {
                self.log.write_commit(tx_id).await?;
                transactions.remove(&tx_id);
                return Ok(());
            }
        }
        Err(super::super::error::Error::TransactionNotFound(tx_id))
    }

    pub async fn rollback(&self, tx_id: Uuid) -> Result<()> {
        let mut transactions = self.active_transactions.write().await;
        if transactions.remove(&tx_id).is_some() {
            self.log.write_rollback(tx_id).await?;
            Ok(())
        } else {
            Err(super::super::error::Error::TransactionNotFound(tx_id))
        }
    }
}
