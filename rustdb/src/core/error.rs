// Contenu de l'ancien workspace_core/src/error.rs

use thiserror::Error;
use std::fmt;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Document introuvable: {0}")]
    NotFound(String),

    #[error("Erreur de validation: {0}")]
    ValidationError(String),

    #[error("Erreur de stockage: {0}")]
    StorageError(#[from] std::io::Error),

    #[error("Erreur de sérialisation: {0}")]
    SerializationError(String),

    #[error("Erreur d'index: {0}")]
    IndexError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
