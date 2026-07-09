use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub struct SchemaValidation {
    pub bson_schema: ValidationSchema,
    pub validation_level: ValidationLevel,
    pub validation_action: ValidationAction,
}

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub struct ValidationSchema {
    pub required: Vec<String>,
    pub properties: HashMap<String, PropertyRule>,
}

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub struct PropertyRule {
    pub bson_type: String,
    pub required: bool,
}

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub enum ValidationLevel {
    Off,
    Moderate,
    Strict,
}

#[derive(Debug, Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
pub enum ValidationAction {
    Error,
    Warn,
}
