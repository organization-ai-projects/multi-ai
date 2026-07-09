use crate::watcher::WatcherConfig; // Corriger l'import
use crate::versioning::{VersionManager, VersionMeta};
use crate::brain::BrainLearner;
use crate::errors::Result;
use crate::publishing::Publisher;
use crate::webhook::WebhookPayload;
use notify::Event;
use tracing::{info, warn};

pub struct ChangeHandler {
    config: WatcherConfig,
    version_manager: VersionManager,
    brain: Option<BrainLearner>,
    publisher: Option<Publisher>,
}

impl ChangeHandler {
    pub fn new(config: WatcherConfig) -> Self {
        let publisher = if config.publish_target.is_some() {
            Some(Publisher::new(&config.cargo_path))
        } else {
            None
        };

        Self {
            version_manager: VersionManager::new(&config.cargo_path),
            brain: config.brain.clone(),
            publisher,
            config,
        }
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<()> {
        // Déplacer toute la logique de traitement ici
        // depuis l'ancien watcher.rs
        Ok(())
    }

    pub fn cleanup(&mut self) {
        // Nettoyage des ressources
        if let Some(backup) = &self.config.cargo_backup {
            let _ = backup.lock().unwrap().rollback();
        }
    }

    pub async fn apply_version_change(
        &mut self,
        version: &str,
        changes: Vec<String>,
    ) -> crate::errors::Result<()> {
        self.version_manager.patch_version(version)?;

        if let Some(ref publisher) = self.publisher {
            let publish_result = publisher.publish(&self.config.publish_target.clone().unwrap_or_default())?;
            if let crate::publishing::PublishResult::Failure(reason) = publish_result {
                warn!("Échec de la publication : {}", reason);
                return Err(format!("Publication échouée : {}", reason).into());
            }
        }

        if let Some(ref webhook) = self.config.webhook {
            webhook.notify_all::<crate::webhook::WebhookPayload>(crate::webhook::WebhookPayload {
                project: self.config.cargo_path.display().to_string(),
                version: version.to_string(),
                changes: changes.clone(),
                author: std::env::var("USER").unwrap_or_default(),
                timestamp: chrono::Utc::now().timestamp() as u64,
            }).await?;
        }

        Ok(())
    }
}
