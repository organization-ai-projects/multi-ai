use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SchemaValidation {
    pub rules: HashMap<String, FieldRule>,
    pub level: ValidationLevel,
    pub action: ValidationAction,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FieldRule {
    pub field_type: FieldType,
    pub required: bool,
    pub unique: bool,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub pattern: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Array,
    Object,
    Date,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ValidationLevel {
    Strict,
    Moderate,
    Off,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ValidationAction {
    Error,
    Warn,
    Skip,
}
