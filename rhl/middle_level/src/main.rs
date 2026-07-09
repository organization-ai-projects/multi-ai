// Modules internes du projet
use middle_level::transpiler::{list_dsl_files, transpile_file};
use middle_level::utils::get_project_root;
use middle_level::types::Format;
use middle_level::linter::DslLinter;

use std::env;
use std::path::Path;

/// Documentation pour la transpilation
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _args: Vec<String> = env::args().collect();
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    
    // Utiliser get_project_root
    let alt_project_root = get_project_root();
    println!("Project root alternatives: {:?} vs {:?}", 
        manifest_dir.parent().unwrap(),
        alt_project_root);

    let dsl_dir = manifest_dir.parent().unwrap().join("dsl_projects");  // Définition de dsl_dir
    
    // Lecture des fichiers de configuration
    let ron_path = manifest_dir.join("functions.ron");
    let types_path = manifest_dir.join("types_mapping.ron");
    
    let format_str = std::fs::read_to_string(&ron_path).expect("Échec lecture functions.ron");
    let types_str = std::fs::read_to_string(&types_path).expect("Échec lecture types_mapping.ron");
    
    let formats: Vec<Format> = ron::from_str(&format_str).expect("RON invalide");
    let type_mappings = ron::from_str(&types_str).expect("Types mapping invalide");

    let out_dir = manifest_dir.parent().unwrap().join("generated");
    std::fs::create_dir_all(&out_dir).expect("Échec création dossier generated");

    println!("🔍 Recherche des fichiers DSL...");
    let files = list_dsl_files(&dsl_dir);
    
    if files.is_empty() {
        println!("❌ Aucun fichier DSL trouvé dans {}", dsl_dir.display());
        return Ok(());
    }

    let mut has_error = false;
    println!("🚀 Transpilation de {} fichiers...", files.len());
    for dsl_path in files {
        let content = std::fs::read_to_string(&dsl_path)?;
        
        // Linting avant transpilation
        let linter = DslLinter::new(&formats, &type_mappings);
        let errors = linter.lint(&content);
        
        if !errors.is_empty() {
            has_error = true;
            println!("❌ Erreurs de linting dans {}:", dsl_path.display());
            for err in errors {
                println!("  Ligne {}, Col {}: {} ({})", 
                    err.line, err.column, err.message, err.code);
            }
            continue;
        }

        if let Err(e) = transpile_file(&dsl_path, &formats, &type_mappings, &out_dir, manifest_dir.parent().unwrap()) {
            has_error = true;
            println!("❌ Erreur sur {}:\n{}", dsl_path.display(), e);
        }
    }
    
    if !has_error {
        println!("✨ Transpilation terminée -> dossier 'generated'");
    }
    Ok(())
}
