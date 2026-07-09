use rand::Rng;
use std::collections::HashMap;

#[derive(Clone)]
pub struct CrossoverStrategy {
    pub crossover_rate: f64,
}

impl CrossoverStrategy {
    pub fn new(crossover_rate: f64) -> Self {
        Self { crossover_rate }
    }

    pub fn crossover(&self, codes: &[String], rng: &mut impl Rng) -> String {
        let mut result = String::new();
        let functions: Vec<HashMap<String, String>> = codes
            .iter()
            .map(|code| self.split_into_functions(code))
            .collect();

        let all_functions: Vec<_> = functions.iter().flat_map(|f| f.keys()).collect();

        for func_name in all_functions {
            if rng.gen_bool(self.crossover_rate) {
                // Utilisation de rng pour décider aléatoirement
                if let Some(func) = functions.iter().filter_map(|f| f.get(func_name)).next() {
                    result.push_str(func);
                    result.push_str("\n\n");
                }
            }
        }

        result
    }

    fn split_into_functions(&self, code: &str) -> HashMap<String, String> {
        let mut functions = HashMap::new();
        let mut current_func = String::new();
        let mut current_name = String::new();
        let mut brace_count = 0;

        for line in code.lines() {
            if line.trim().starts_with("fn ") {
                if !current_name.is_empty() && !current_func.is_empty() {
                    functions.insert(current_name.clone(), current_func.clone());
                }
                current_name = line.split_whitespace().nth(1).unwrap_or("").to_string();
                current_func.clear();
            }

            current_func.push_str(line);
            current_func.push('\n');

            brace_count += line.matches('{').count();
            brace_count -= line.matches('}').count();

            if brace_count == 0 && !current_name.is_empty() {
                functions.insert(current_name.clone(), current_func.clone());
                current_name.clear();
                current_func.clear();
            }
        }

        if !current_name.is_empty() && !current_func.is_empty() {
            functions.insert(current_name, current_func);
        }

        functions
    }
}
