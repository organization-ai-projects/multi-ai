use std::collections::HashSet;
pub use crate::types::Format;  // Uniquement la réexportation publique

pub fn generate_imports(formats: &[Format]) -> String {
    let mut std_modules = HashSet::new();
    let mut ext_modules = HashSet::new();
    
    // Collecter tous les modules
    for fmt in formats {
        std_modules.extend(fmt.std_modules.iter().cloned());
        ext_modules.insert(fmt.module.split("::").next().unwrap().to_string());
    }

    let mut imports = String::new();

    // Générer les imports standards
    let std_list: Vec<_> = std_modules.iter().collect();
    for module in std_list {
        imports.push_str(&format!("use {}; // Standard library import\n", module));
    }
    imports.push_str("use serde::Serialize;\n\n");

    // Générer les imports externes de manière ordonnée
    let mut ext_list: Vec<_> = ext_modules.into_iter().collect();
    ext_list.sort();
    for module in ext_list {
        imports.push_str(&format!("use {}; // External crate import\n", module));
    }
    imports.push('\n');

    // Ajouter l'import de Arc si nécessaire
    if std_modules.iter().any(|m| m.contains("Arc")) {
        imports.push_str("use std::sync::Arc;\n");
    }

    imports
}
