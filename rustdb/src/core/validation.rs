use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct SchemaValidation {
    pub rules: HashMap<String, FieldRule>,
    pub level: ValidationLevel,
    pub action: ValidationAction,
}

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub struct FieldRule {
    pub field_type: FieldType,
    pub required: bool,
    pub unique: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Date,
}

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum ValidationLevel {
    Strict,
    Moderate,
    Off,
}

#[derive(Debug, Deserialize, Serialize, Clone, bincode_next::Encode, bincode_next::Decode)]
pub enum ValidationAction {
    Error,
    Warn,
    Skip,
}
