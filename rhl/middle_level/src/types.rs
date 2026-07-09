use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transform {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct TransformFunction {
    pub params: Vec<String>,     // Référence les types depuis types_mapping.ron
    pub return_type: String,     // Référence un type depuis types_mapping.ron
    pub lang_format: String,
}

#[derive(Debug, Deserialize)]
/// Représente une transformation de type DSL, voir [`DslType`] pour plus d'infos
pub struct TypeTransform {
    pub dsl_type: String,      // Type brut à parser
    pub rust_type: Option<String>, // Override du type Rust si nécessaire
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    pub args: Vec<String>,
    pub return_type: Option<String>,
    pub body: Vec<String>
}

#[derive(Debug, Deserialize)]
pub struct Format {
    pub name: String,
    pub module: String,
    pub function: String,
    pub args: Vec<String>,
    pub return_handling: String,
    pub extension: String,
    #[serde(default)]
    pub output_mode: OutputMode,
    #[serde(default)]
    pub std_modules: Vec<String>,
    #[serde(default)]
    pub transform: Option<Transform>,
    #[serde(default)]
    pub transform_function: Option<TransformFunction>, // Doit être Option<>
    #[serde(default)]
    pub type_transform: Option<TypeTransform>,
}

#[derive(Debug, Deserialize, Default)]
pub enum OutputMode {
    #[default]
    File,
    String,
}

// Implémentation pour utiliser Transform
impl Transform {
    pub fn apply(&self, input: &str) -> String {
        if input == self.from {
            self.to.clone()
        } else {
            input.to_string()
        }
    }
}

// Implémentation pour utiliser TransformFunction
impl TransformFunction {
    pub fn transform_params(&self) -> Vec<String> {
        self.params.clone()
    }
    
    pub fn get_return(&self) -> String {
        self.return_type.clone()
    }
}

// Implémentation pour utiliser TypeTransform
impl TypeTransform {
    pub fn to_rust_type(&self) -> String {
        self.rust_type.clone().unwrap_or_else(|| self.dsl_type.clone())
    }
}

// Implémentation pour utiliser Format
impl Format {
    pub fn get_effective_type(&self) -> String {
        if let Some(tt) = &self.type_transform {
            tt.to_rust_type()
        } else {
            "()".to_string()
        }
    }
}
