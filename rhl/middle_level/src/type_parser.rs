use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Modifier {
    Ref,
    Mut,
    Box,
    None,
}

impl std::fmt::Display for Modifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Modifier::Ref => write!(f, "Ref"),
            Modifier::Mut => write!(f, "Mut"),
            Modifier::Box => write!(f, "Box"),
            Modifier::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DslType {
    pub base: String,
    pub modifier: Modifier,
    pub generic: Option<Box<DslType>>,
}

#[derive(Debug, Deserialize)]
pub struct TypesMapping {
    pub source_lang: String,
    pub types: HashMap<String, TypeDefinition>,
    pub modifiers: HashMap<String, ModifierDefinition>,
    pub syntax: SyntaxDefinition,
}

#[derive(Debug, Deserialize)]
pub struct TypeDefinition {
    pub from: HashMap<String, String>,
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct ModifierDefinition {
    pub rust: String,
    pub modifier_type: String,
    pub keyword: String,  // Ajout du champ manquant
}

impl ModifierDefinition {
    pub fn to_modifier(&self) -> Modifier {
        match self.modifier_type.as_str() {
            "Mut" => Modifier::Mut,
            "Ref" => Modifier::Ref,
            "Box" => Modifier::Box,
            _ => Modifier::None
        }
    }
}

impl std::fmt::Display for ModifierDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.rust)
    }
}

impl DslType {
    pub fn to_rust_with_mapping(&self, mappings: &TypesMapping) -> String {
        let base_rust = if let Some(type_def) = mappings.types.get(&self.base) {
            // Utiliser le champ from
            if let Some(from_type) = type_def.from.get(&mappings.source_lang) {
                format!("/* From {} */ {}", from_type, type_def.to)
            } else {
                type_def.to.clone()
            }
        } else {
            self.base.clone()
        };

        let mut result = base_rust;

        // Appliquer le générique si présent
        if let Some(inner) = &self.generic {
            let inner_rust = inner.to_rust_with_mapping(mappings);
            result = format!("{}<{}>", result, inner_rust);
        }

        // Appliquer le modifier depuis le mapping
        if let Some(modifier_map) = mappings.modifiers.get(&self.modifier.to_string()) {
            format!("{}{}", modifier_map.rust, result)  // Utiliser .rust au lieu du Display
        } else {
            result
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct SyntaxDefinition {
    pub modifiers: Vec<ModifierDefinition>,
}

impl TypesMapping {
    pub fn get_modifier(&self, keyword: &str) -> Modifier {
        if let Some(modifier_def) = self.modifiers.get(keyword) {
            match modifier_def.modifier_type.as_str() {
                "Mut" => Modifier::Mut,
                "Ref" => Modifier::Ref,
                "Box" => Modifier::Box,
                _ => Modifier::None
            }
        } else {
            Modifier::None
        }
    }
}

pub fn parse_dsl_type(dsl: &str, mappings: &TypesMapping) -> DslType {
    let parts: Vec<&str> = dsl.trim().split_whitespace().collect();
    
    // Récupère les parties selon leur position
    let (modifier, type_name) = match parts.len() {
        // nom_type nom_var
        2 => (Modifier::None, parts[0]),
        // modificateur nom_type nom_var
        3 => {
            let modifier = mappings.syntax.modifiers.iter()
                .find(|m| m.keyword == parts[0])
                .map(|m| m.to_modifier())
                .unwrap_or(Modifier::None);
            (modifier, parts[1])
        },
        _ => (Modifier::None, parts[0])
    };

    if let Some(start) = type_name.find('<') {
        let end = type_name.rfind('>').unwrap();
        let base = &type_name[..start];
        let inner = &type_name[start + 1..end];
        DslType {
            base: base.to_string(),
            modifier,
            generic: Some(Box::new(parse_dsl_type(inner, mappings)))
        }
    } else {
        DslType {
            base: type_name.to_string(),
            modifier,
            generic: None
        }
    }
}
