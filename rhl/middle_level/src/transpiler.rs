use std::fs::{read_dir, read_to_string, write};
use std::path::{Path, PathBuf};
use crate::types::{Format, Function};  // Suppression de OutputMode inutilisé
use crate::formats::generate_imports;
use crate::type_parser::TypesMapping;  // Suppression de parse_dsl_type inutilisé
use crate::ast::{parse_function, Function as AstFunction};

pub fn list_dsl_files(dir: &Path) -> Vec<PathBuf> {
    let mut dsl_files = Vec::new();
    if let Ok(entries) = read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                dsl_files.extend(list_dsl_files(&path));
            } else if path.extension().map_or(false, |ext| ext == "rhl") {  // Change dsl to rhl
                dsl_files.push(path);
            }
        }
    }
    dsl_files
}

fn find_path_and_data<'a>(args: &'a [&'a str]) -> (Option<&'a str>, Option<&'a str>) {
    let mut path = None;
    let mut data = None;
    
    for arg in args {
        if arg.starts_with("path:") {
            path = Some(&arg[5..]); // Skip "path:"
        } else {
            data = Some(*arg); // Déréférencement pour obtenir &str au lieu de &&str
        }
    }
    
    (path, data)
}

fn validate_and_get_path<'a>(fmt: &Format, args: &'a [&'a str]) -> Result<(String, &'a str), Box<dyn std::error::Error>> {
    let (path, data) = find_path_and_data(args);
    
    let data = data.ok_or_else(|| format!("Erreur: donnée manquante pour {}", fmt.name))?;
    let path = path.ok_or_else(|| 
        format!("Erreur: chemin manquant pour {}. Utilisez path:nom_fichier", fmt.name))?;

    // Retirer les guillemets si présents
    let clean_path = path.trim_matches('"');

    if let Some(provided_ext) = Path::new(clean_path).extension() {
        if provided_ext.to_string_lossy() != fmt.extension {
            return Err(format!("Erreur: l'extension doit être .{} pour {}, pas .{}", 
                fmt.extension, fmt.name, provided_ext.to_string_lossy()).into());
        }
        Ok((clean_path.to_string(), data))
    } else {
        Ok((format!("{}.{}", clean_path, fmt.extension), data))
    }
}

fn has_imports(content: &str) -> bool {
    content.lines().any(|line| line.trim().starts_with("use "))
}

fn generate_output_handling(fmt: &Format, args_str: &str, _rel_path: &Path) -> String {
    format!(
        "    {}::{}({}){};\n",
        fmt.module, fmt.function, args_str, fmt.return_handling
    )
}

fn generate_functions(functions: &[Function]) -> String {
    let mut code = String::new();
    
    for func in functions {
        // Générer la signature
        let ret = func.return_type.as_deref().unwrap_or("()");
        code.push_str(&format!("fn {}({}) -> {} {{\n", 
            func.name,
            func.args.join(", "),
            ret
        ));
        
        // Générer le corps
        for line in &func.body {
            code.push_str(&format!("    {}\n", line));
        }
        code.push_str("}\n\n");
    }
    code
}

pub fn transpile_file(dsl_path: &Path, formats: &[Format], type_mappings: &TypesMapping, out_dir: &Path, project_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let rel_path = dsl_path.strip_prefix(project_root.join("dsl_projects"))?;
    let out_path = out_dir.join(rel_path).with_extension("rs");
    
    let dsl_str = read_to_string(dsl_path)?;
    let mut rust_code = String::new();

    // Imports - Ne garder que les imports réellement nécessaires
    let clean_imports = generate_imports(formats)
        .lines()
        .filter(|line| !line.contains("rhl"))  // Supprimer tout ce qui est lié à rhl
        .filter(|line| !line.trim().is_empty())  // Supprimer les lignes vides
        .collect::<Vec<_>>()
        .join("\n");

    if !clean_imports.trim().is_empty() {
        rust_code.push_str(&clean_imports);
        rust_code.push('\n');
    }

    // Traitement ligne par ligne du DSL
    for line in dsl_str.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with("serialize_") || line.starts_with("deserialize_") {
            if let Some(paren) = line.find('(') {
                let end = line.rfind(')').unwrap_or(line.len());
                let name = line[..paren].trim();
                if let Some(fmt) = formats.iter().find(|f| f.name == name) {
                    let args = &line[paren + 1..end];
                    let args_cleaned: Vec<&str> = args.split(',').map(|s| s.trim()).collect();
                    
                    let (path, data) = validate_and_get_path(fmt, &args_cleaned)?;
                    let args_joined = fmt.args.join(", ")
                        .replace("\"&path\"", &format!("\"{}\"", path))
                        .replace("&input", data);
                    rust_code.push_str(&format!("    {}::{}({}){};\n", 
                        fmt.module, fmt.function, args_joined, fmt.return_handling));
                }
            }
        } else if line.starts_with("function ") {
            // Extraction de la fonction complète
            let func_lines = dsl_str.lines()
                .skip_while(|l| !l.starts_with("function "))
                .take_while(|l| !l.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n");

            if let Some(func) = parse_function(&func_lines) {
                // Conversion des types
                let converted_func = AstFunction {
                    name: func.name,
                    params: func.params.into_iter()
                        .map(|(name, typ)| {
                            let rust_type = match typ.as_str() {
                                "all_number" | "int" => "i32",
                                _ => type_mappings.types
                                    .get(&typ)
                                    .map(|t| t.to.as_str())
                                    .unwrap_or("i32")
                            };
                            (name, rust_type.to_string())
                        })
                        .collect(),
                    return_type: func.return_type,
                    body: func.body,
                };
                rust_code.push_str(&converted_func.to_rust());
            }
        }
    }

    // Écriture directe sans commentaires de debug
    write(&out_path, rust_code)?;
    Ok(())
}