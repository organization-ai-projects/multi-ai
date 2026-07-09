use std::{fs, path::Path, process::Command};

#[derive(Clone)]
pub struct MutationValidator {
    cache: std::collections::HashMap<String, bool>,
    failed_mutations: usize,
}

impl MutationValidator {
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            failed_mutations: 0,
        }
    }

    pub fn verify_mutation(&mut self, code: &str, path: &Path) -> bool {
        let hash = fast_hash(code);
        if let Some(&result) = self.cache.get(&hash) {
            return result;
        }

        let result = self.perform_verification(code, path);

        if !result {
            self.failed_mutations += 1;
        }
        self.cache.insert(hash, result);

        if self.cache.len() > 1000 {
            self.cache.clear();
        }

        result
    }

    fn perform_verification(&self, code: &str, path: &Path) -> bool {
        if !self.verify_syntax(code) {
            return false;
        }

        if !self.verify_compilation(code, path) {
            return false;
        }

        true
    }

    fn verify_syntax(&self, code: &str) -> bool {
        syn::parse_file(code).is_ok()
    }

    fn verify_compilation(&self, code: &str, path: &Path) -> bool {
        if let Ok(_) = fs::write(path, code) {
            Command::new("cargo")
                .arg("check")
                .current_dir(path.parent().unwrap())
                .output()
                .map(|output| output.status.success())
                .unwrap_or(false)
        } else {
            false
        }
    }

    pub fn get_failed_mutations(&self) -> usize {
        self.failed_mutations
    }

    pub fn reset_failed_mutations(&mut self) {
        self.failed_mutations = 0;
    }
}

fn fast_hash(s: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish().to_string()
}
