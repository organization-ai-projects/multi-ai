use crate::types::Format;
use crate::type_parser::TypesMapping;

#[derive(Debug)]
pub struct LintError {
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub code: String,
}

pub struct DslLinter<'a> {
    formats: &'a [Format],
    type_mappings: &'a TypesMapping,
}

impl<'a> DslLinter<'a> {
    pub fn new(formats: &'a [Format], type_mappings: &'a TypesMapping) -> Self {
        Self { formats, type_mappings }
    }

    pub fn lint(&self, content: &str) -> Vec<LintError> {
        let mut errors = Vec::new();
        
        for (line_num, line) in content.lines().enumerate() {
            let line_num = line_num + 1;
            let line = line.trim();
            
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if line.starts_with("serialize_") || line.starts_with("deserialize_") {
                self.check_serialization(line, line_num, &mut errors);
            } 
            else if line == "function" {
                errors.push(LintError {
                    line: line_num,
                    column: 0,
                    message: "Structure minimale attendue pour 'function':\n\
                        function nom_fonction() { }  // Sans paramètres ni retour\n\
                        \n\
                        Options possibles:\n\
                        - Paramètres: function nom(param1: type1, param2: type2)\n\
                        - Type retour: function nom() -> type_retour\n\
                        - Corps: function nom() { instructions... }\n\
                        \n\
                        Exemple complet:\n\
                        function calcul_prix(quantite: all_number, prix: decimal_number) -> decimal_number {\n\
                            // instructions...\n\
                        }".into(),
                    code: "E006".into(),
                });
            }
            else if line.starts_with("function ") {  // Ajout de la vérification des fonctions
                self.check_function(line, line_num, &mut errors);
            }
            else if !line.starts_with("//") && !line.is_empty() {
                self.check_general_syntax(line, line_num, &mut errors);
            }
        }
        errors
    }

    fn check_serialization(&self, line: &str, line_num: usize, errors: &mut Vec<LintError>) {
        // Vérifier la présence des parenthèses
        if !line.contains('(') || !line.contains(')') {
            errors.push(LintError {
                line: line_num,
                column: 0,
                message: "Format invalide - utilisez: serialize_*(path:file, data)".into(),
                code: "E003".into(),
            });
            return;
        }

        // Vérifier le format des arguments
        let open_paren = line.find('(').unwrap();
        let close_paren = line.find(')').unwrap();
        let args_str = &line[open_paren + 1..close_paren];
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();
        
        // Vérifier la fonction
        let fn_name = &line[..open_paren];
        if !self.formats.iter().any(|f| f.name == fn_name) {
            errors.push(LintError {
                line: line_num,
                column: 0,
                message: format!("Fonction inconnue: {}", fn_name),
                code: "E010".into(),
            });
        }
        
        if args.len() != 2 {
            errors.push(LintError {
                line: line_num,
                column: line.find('(').unwrap(),
                message: "Nombre d'arguments incorrect - attendu: 2 (path et data)".into(),
                code: "E008".into(),
            });
        }

        // Vérifier le format path:
        if !args.iter().any(|arg| arg.starts_with("path:")) {
            errors.push(LintError {
                line: line_num,
                column: line.find('(').unwrap(),
                message: "Le chemin doit être défini avec 'path:' suivi d'une chaîne de caractères, ex: path:\"mon_fichier\"".into(),
                code: "E009".into(),
            });
        }

        // Vérification des valeurs path:
        if let Some(path_arg) = args.iter().find(|arg| arg.starts_with("path:")) {
            let path_value = &path_arg[5..];
            if !path_value.starts_with('"') || !path_value.ends_with('"') {
                errors.push(LintError {
                    line: line_num,
                    column: line.find(path_value).unwrap_or(0),
                    message: "Les chemins doivent être entre guillemets, ex: path:\"mon_fichier.ron\"".into(),
                    code: "E011".into(),
                });
            }
        }
    }

    fn check_function(&self, line: &str, line_num: usize, errors: &mut Vec<LintError>) {
        // Vérifier la syntaxe de base d'une fonction
        let function_pattern = r"^function\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\((.*)\)\s*(?:->\s*([a-zA-Z_][a-zA-Z0-9_]*))?";
        if !regex::Regex::new(function_pattern).unwrap().is_match(line) {
            errors.push(LintError {
                line: line_num,
                column: 0,
                message: "Syntaxe de fonction invalide".into(),
                code: "E012".into(),
            });
            return;
        }

        // Vérifier les types des paramètres
        if let Some(params_str) = line.split('(').nth(1).and_then(|s| s.split(')').next()) {
            for param in params_str.split(',') {
                let parts: Vec<&str> = param.trim().split(':').collect();
                if parts.len() == 2 {
                    let type_name = parts[1].trim();
                    if type_name == "int" {
                        errors.push(LintError {
                            line: line_num,
                            column: line.find(type_name).unwrap_or(0),
                            message: "Type 'int' n'existe pas en RHL. Utilisez 'all_number' pour un entier signé ou 'positive_only' pour un entier positif".into(),
                            code: "E011".into(),
                        });
                    } else if !self.type_mappings.types.contains_key(type_name) {
                        errors.push(LintError {
                            line: line_num,
                            column: line.find(type_name).unwrap_or(0),
                            message: format!("Type '{}' inconnu. Types disponibles: all_number, positive_only, decimal_number, etc.", type_name),
                            code: "E011".into(),
                        });
                    }
                }
            }
        }
    }

    fn check_general_syntax(&self, line: &str, line_num: usize, errors: &mut Vec<LintError>) {
        // Accepter les accolades seules et les instructions de base
        if line == "{" || line == "}" || line.starts_with("return ") {
            return;
        }
        
        errors.push(LintError {
            line: line_num,
            column: 0,
            message: format!("Ligne non reconnue: {}", line),
            code: "E007".into(),
        });
    }
}
