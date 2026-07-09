use rand::{self, prelude::SliceRandom, Rng};
use std::collections::{HashMap, HashSet};

#[derive(Clone)] // Ajout du trait Clone
pub struct CrossoverStrategy {
    pub crossover_rate: f64,
}

impl CrossoverStrategy {
    pub fn new(crossover_rate: f64) -> Self {
        Self { crossover_rate }
    }

    pub fn crossover(&self, codes: &[String], rng: &mut impl Rng) -> String {
        let mut result = String::new();
        let functions: Vec<HashMap<_, _>> = codes
            .iter()
            .map(|code| self.split_into_functions(code))
            .collect();

        // Mélange de fonctions de différentes sources
        let all_functions: HashSet<_> = functions.iter().flat_map(|f| f.keys()).collect();

        for func_name in all_functions {
            let source_idx = rng.random_range(0..functions.len());
            if let Some(func) = functions[source_idx].get(func_name) {
                // 30% chance de faire un crossover interne
                if rng.random_bool(0.3) {
                    result.push_str(&self.cross_function_bodies(
                        func,
                        functions.iter().filter_map(|f| f.get(func_name)).collect(),
                    ));
                } else {
                    result.push_str(func);
                }
                result.push_str("\n\n");
            }
        }

        result
    }

    fn cross_function_bodies(&self, base: &str, variants: Vec<&String>) -> String {
        let mut rng = rand::rng();
        let mut lines: Vec<_> = base.lines().collect();

        // Identifier les blocs { } dans la fonction
        let block_starts: Vec<usize> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| line.contains('{'))
            .map(|(i, _)| i)
            .collect();

        if let Some(&block_start) = block_starts.choose(&mut rng) {
            let block_end = find_matching_brace(&lines, block_start);
            if let Some(end) = block_end {
                // 30% de chance de remplacer le bloc
                if rng.random_bool(0.3) {
                    if let Some((v_start, v_end)) =
                        find_similar_block(&variants[0].lines().collect::<Vec<_>>())
                    {
                        // Remplacer le bloc par celui de la variante
                        lines.splice(
                            block_start..=end,
                            variants[0].lines().skip(v_start).take(v_end - v_start + 1),
                        );
                    }
                }
            }
        }

        lines.join("\n")
    }

    fn split_into_functions(&self, code: &str) -> HashMap<String, String> {
        let mut functions = HashMap::new();
        let mut current_func = String::new();
        let mut current_name = String::new();
        let mut brace_count = 0;
        let mut in_function = false;

        for line in code.lines() {
            if line.trim().starts_with("fn ") {
                // Nouvelle fonction trouvée
                if !current_name.is_empty() && !current_func.is_empty() {
                    functions.insert(current_name, current_func);
                }
                current_name = extract_function_name(line);
                current_func = String::new();
                in_function = true;
            }

            if in_function {
                current_func.push_str(line);
                current_func.push('\n');

                brace_count += line.matches('{').count();
                brace_count -= line.matches('}').count();

                if brace_count == 0 && !current_func.is_empty() {
                    // Fin de la fonction
                    functions.insert(current_name.clone(), current_func.clone());
                    current_func = String::new();
                    current_name = String::new();
                    in_function = false;
                }
            }
        }

        // Ajouter la dernière fonction si nécessaire
        if !current_name.is_empty() && !current_func.is_empty() {
            functions.insert(current_name, current_func);
        }

        functions
    }
}

// Fonctions utilitaires pour le parsing
fn extract_function_name(line: &str) -> String {
    line.split("fn ")
        .nth(1)
        .and_then(|s| s.split('(').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_default()
}

fn find_matching_brace(lines: &[&str], start: usize) -> Option<usize> {
    let mut count = 0;
    for (i, line) in lines.iter().enumerate().skip(start) {
        count += line.matches('{').count();
        count -= line.matches('}').count();
        if count == 0 {
            return Some(i);
        }
    }
    None
}

fn find_similar_block(lines: &[&str]) -> Option<(usize, usize)> {
    for (i, line) in lines.iter().enumerate() {
        if line.contains("{") {
            if let Some(end) = find_matching_brace(lines, i) {
                return Some((i, end));
            }
        }
    }
    None
}
