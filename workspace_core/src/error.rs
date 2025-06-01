use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Document non trouvé: {0}")]
    NotFound(String),
    
    #[error("Erreur de validation: {0}")]
    ValidationError(String),
    
    #[error("Erreur de stockage: {0}")]
    StorageError(#[from] std::io::Error),
    
    #[error("Erreur de sérialisation: {0}")]
    SerializationError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
