pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AutoVersionError {
    #[error("Fichier non trouvé: {0}")]
    FileNotFound(String),
    
    #[error("Erreur de configuration: {0}")]
    ConfigError(String),
    
    #[error("Erreur de publication: {0}")]
    PublishError(String),
}
