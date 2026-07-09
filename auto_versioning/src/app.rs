use std::sync::Arc;
use crate::api;
use crate::errors::Result;
use crate::setup::AppComponents;
use crate::watcher::VersionWatcher;
use tracing::{info, warn};
use std::sync::Mutex as StdMutex;

pub struct Application {
    components: AppComponents,
    watcher: Arc<StdMutex<VersionWatcher>>,
}

impl Application {
    pub fn new(components: AppComponents) -> Self {
        let watcher = Arc::new(StdMutex::new(
            VersionWatcher::new(components.watcher_config.clone())
        ));

        Self { 
            components,
            watcher,
        }
    }

    pub async fn run(&self) -> Result<()> {
        // Démarrer l'API
        self.start_api_server().await?;
        
        // Démarrer le watcher
        self.start_watcher()?;
        
        Ok(())
    }

    async fn start_api_server(&self) -> Result<()> {
        let api_state = self.components.api_state.clone();
        tokio::spawn(async move {
            api::start_api_server(api_state).await;
        });
        Ok(())
    }

    fn start_watcher(&self) -> Result<()> {
        let mut watcher = self.watcher.lock().unwrap();
        watcher.watch(&self.components.watcher_config.root_path)?;
        Ok(())
    }
}
