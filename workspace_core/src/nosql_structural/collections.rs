use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::references::DbRef;
use std::collections::HashMap;

// Structure de base d'une collection dans notre système
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Collection<T> {
    pub _id: Uuid,
    pub name: String,
    pub documents: Vec<Document<T>>,
    pub indexes: HashMap<String, Index>,
    pub references: Vec<DbRef>,
}

// Structure de base d'un document
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Document<T> {
    pub _id: Uuid,
    pub created_at: i64,
    pub data: T,            // T est le schéma défini dans schema/
    pub references: Option<Vec<DbRef>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Index {
    pub field: String,
    pub unique: bool,
    pub sparse: bool,
}
