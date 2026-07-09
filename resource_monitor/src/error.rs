use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResourceMonitorError {
    #[error("Erreur de surveillance: {0}")]
    MonitoringError(String),
    
    #[error("Seuil de ressources dépassé: {0}")]
    ThresholdExceeded(String),
    
    #[error("Erreur de configuration: {0}")]
    ConfigurationError(String),
    
    #[error("Violation de sécurité: {0}")]
    SecurityViolation(String),
}
