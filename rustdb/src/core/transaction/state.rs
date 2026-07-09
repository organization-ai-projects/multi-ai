use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TransactionState {
    Active,
    Committing,
    Committed,
    RollingBack,
    RolledBack,
}

#[derive(Debug)]
pub struct TransactionResult {
    pub id: Uuid,
    pub state: TransactionState,
    pub duration: std::time::Duration,
}
