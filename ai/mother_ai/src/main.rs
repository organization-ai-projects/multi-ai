mod memory;
mod lifecycle;
mod training;
mod common;

use lifecycle::LifecycleManager;
use std::error::Error;
use std::path::Path;
use common::logging::{configure_global_logger, LogLevel};

fn main() -> Result<(), Box<dyn Error>> {
    // Initialiser le logger
    configure_global_logger(LogLevel::Debug, "MOTHER_AI");
    crate::log_info!("🧠 Démarrage de l'IA Mother");

    // Vérifier si la config de l'IA existe (optionnel)
    let ia_name = "mother_ai";

    // Initialiser le gestionnaire de cycle de vie
    let mut lifecycle = LifecycleManager::new(ia_name.to_string())?;

    // Lancer le cycle
    lifecycle.run()?;

    Ok(())
}
