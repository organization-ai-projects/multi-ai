//! Module de gestion des erreurs personnalisées
//! 
//! Fournit des types d'erreurs spécifiques pour différentes parties de l'application,
//! permettant une gestion plus précise et des messages d'erreur plus informatifs.

use std::error::Error;
use std::fmt;
use std::io;

/// Erreur liée aux opérations de mémoire
#[derive(Debug)]
pub enum MemoryError {
    /// Erreur lors de l'accès à un fichier
    IoError(io::Error),
    /// Erreur de désérialisation
    DeserializationError(String),
    /// Erreur de sérialisation
    SerializationError(String),
    /// Configuration introuvable
    ConfigNotFound(String),
    /// Format de fichier non supporté
    UnsupportedFormat(String),
    /// Noeud non trouvé dans le graphe de mémoire
    NodeNotFound(String),
    /// Chemin de mémoire invalide
    InvalidPath(String),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::IoError(err) => write!(f, "Erreur I/O: {}", err),
            MemoryError::DeserializationError(msg) => write!(f, "Erreur de désérialisation: {}", msg),
            MemoryError::SerializationError(msg) => write!(f, "Erreur de sérialisation: {}", msg),
            MemoryError::ConfigNotFound(name) => write!(f, "Configuration non trouvée pour l'IA: {}", name),
            MemoryError::UnsupportedFormat(format) => write!(f, "Format non supporté: {}", format),
            MemoryError::NodeNotFound(id) => write!(f, "Noeud non trouvé: {}", id),
            MemoryError::InvalidPath(path) => write!(f, "Chemin invalide: {}", path),
        }
    }
}

impl Error for MemoryError {}

impl From<io::Error> for MemoryError {
    fn from(error: io::Error) -> Self {
        MemoryError::IoError(error)
    }
}

/// Résultat personnalisé pour les opérations de mémoire
pub type MemoryResult<T> = Result<T, MemoryError>;

/// Extension sur Result pour convertir les erreurs io::Error en MemoryError
pub trait IoResultExt<T> {
    /// Convertit une io::Error en MemoryError avec un message contextuel
    fn context(self, context: &str) -> MemoryResult<T>;
}

impl<T> IoResultExt<T> for io::Result<T> {
    fn context(self, context: &str) -> MemoryResult<T> {
        self.map_err(|err| {
            let details = format!("{}: {}", context, err);
            MemoryError::IoError(io::Error::new(err.kind(), details))
        })
    }
}
