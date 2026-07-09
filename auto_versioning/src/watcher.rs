use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use notify::{RecursiveMode, Watcher};
use tracing::{info, warn};

use crate::monitoring::change_handler::ChangeHandler;
use crate::watcher::WatcherConfig; // Corriger l'import

pub struct VersionWatcher {
    watcher: notify::PollWatcher,
    pub config: WatcherConfig, // Rendre config public
    change_handler: Arc<Mutex<ChangeHandler>>,
}

impl VersionWatcher {
    pub fn new(config: WatcherConfig) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let change_handler = Arc::new(Mutex::new(ChangeHandler::new(config.clone())));
        let handler = change_handler.clone();

        let watcher = notify::PollWatcher::new(
            move |res| {
                if let Ok(event) = res {
                    if let Err(e) = handler.lock().unwrap().handle_event(&event) {
                        warn!("Erreur lors du traitement: {}", e);
                    }
                }
            },
            notify::Config::default(),
        ).expect("Échec initialisation watcher");

        Self {
            watcher,
            config,
            change_handler,
        }
    }

    pub fn watch(&mut self, base_dir: &PathBuf) -> crate::errors::Result<()> {
        info!("🔍 Surveille les modifications dans {}", base_dir.display());
        self.watcher.watch(base_dir, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn stop(&mut self) {
        info!("Arrêt de la surveillance");
        self.change_handler.lock().unwrap().cleanup();
    }
}