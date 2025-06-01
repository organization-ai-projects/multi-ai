use std::path::PathBuf;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use chrono::Utc;

pub struct TransactionLog {
    log_path: PathBuf,
}

impl TransactionLog {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            log_path: path.into(),
        }
    }

    pub async fn write_begin(&self, tx_id: Uuid) -> Result<()> {
        self.write_entry(tx_id, "BEGIN").await
    }

    pub async fn write_commit(&self, tx_id: Uuid) -> Result<()> {
        self.write_entry(tx_id, "COMMIT").await
    }

    pub async fn write_rollback(&self, tx_id: Uuid) -> Result<()> {
        self.write_entry(tx_id, "ROLLBACK").await
    }

    async fn write_entry(&self, tx_id: Uuid, action: &str) -> Result<()> {
        let entry = format!(
            "{} - {} - {}\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S%.3f"),
            tx_id,
            action
        );

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)
            .await?;

        file.write_all(entry.as_bytes()).await?;
        file.flush().await?;
        Ok(())
    }
}
