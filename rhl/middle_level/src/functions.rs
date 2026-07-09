use std::collections::HashMap;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, String)>,  // (nom, type)
    pub return_type: Option<String>,
    pub body: Vec<String>,
}

pub struct FunctionRegistry {
    functions: HashMap<String, Function>
}

impl FunctionRegistry {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new()
        }
    }

    pub fn register_function(&mut self, name: &str, params: Vec<(String, String)>, 
                           return_type: Option<String>, body: Vec<String>) -> String {
        let func = Function {
            name: name.to_string(),
            params,
            return_type,
            body,
        };

        // Générer le code Rust
        let mut rust_code = format!("pub fn {} (", name);
        
        // Paramètres
        let params_str = func.params.iter()
            .map(|(name, typ)| format!("{}: {}", name, typ))
            .collect::<Vec<_>>()
            .join(", ");
        rust_code.push_str(&params_str);
        rust_code.push_str(") ");

        // Type de retour
        if let Some(ret) = &func.return_type {
            rust_code.push_str(&format!("-> {} ", ret));
        }

        // Corps
        rust_code.push_str("{\n");
        for line in &func.body {
            rust_code.push_str(&format!("    {}\n", line));
        }
        rust_code.push_str("}\n");

        self.functions.insert(name.to_string(), func);
        rust_code
    }
}

// Singleton pour le registre
lazy_static::lazy_static! {
    static ref REGISTRY: std::sync::Mutex<FunctionRegistry> = std::sync::Mutex::new(FunctionRegistry::new());
}

pub fn register_function(name: &str, params: &[(String, String)], 
                        return_type: &Option<String>, body: &[String]) -> String {
    let mut registry = REGISTRY.lock().unwrap();
    let output = registry.register_function(name, params.to_vec(), return_type.clone(), body.to_vec());

    // Utilisation de tous les champs pour la documentation
    let func = registry.functions.get(name).unwrap();
    format!("/// Function: {}\n/// Params: {:?}\n/// Return: {:?}\n{}", 
            func.name, func.params, func.return_type, output)
}
