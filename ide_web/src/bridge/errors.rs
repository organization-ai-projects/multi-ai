use std::sync::PoisonError;
use std::fmt;

#[derive(Debug)]
pub enum BridgeError {
    Io(std::io::Error),
    Lock(String),
    IA(String),
}

impl BridgeError {
    /// Retourne un code unique pour chaque type d'erreur
    pub fn code(&self) -> &'static str {
        match self {
            BridgeError::Io(_) => "io_error",
            BridgeError::Lock(_) => "lock_error",
            BridgeError::IA(_) => "ia_error",
        }
    }
}

impl From<std::io::Error> for BridgeError {
    fn from(e: std::io::Error) -> Self {
        BridgeError::Io(e)
    }
}

impl<T> From<PoisonError<T>> for BridgeError {
    fn from(_: PoisonError<T>) -> Self {
        BridgeError::Lock("Erreur de verrouillage du mutex".to_string())
    }
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BridgeError::Io(e) => write!(f, "Erreur I/O : {}", e),
            BridgeError::Lock(_) => write!(f, "Erreur de verrouillage mémoire IA"),
            BridgeError::IA(msg) => write!(f, "Erreur IA : {}", msg),
        }
    }
}

/// Fonction pour expliquer les erreurs de manière lisible
pub fn explain_error(error: &BridgeError) -> String {
    match error {
        BridgeError::Io(_) => "Erreur système : accès disque impossible".to_string(),
        BridgeError::Lock(_) => "Erreur interne : IA bloquée par une opération concurrente".to_string(),
        BridgeError::IA(msg) => format!("Erreur IA : {}", msg),
    }
}

/// Alias pour simplifier les signatures des fonctions
pub type BridgeResult<T> = Result<T, String>;
