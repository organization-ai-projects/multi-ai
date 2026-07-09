use super::{MutationProvider, MutationStrategy};
use rand::{prelude::SliceRandom, Rng};
use std::{collections::HashSet, fs, path::Path};
use walkdir::WalkDir;

pub struct MutationWalker {
    known_modules: HashSet<String>,
    mutation_strategy: MutationStrategy,
    population_dir: String,
}

impl MutationWalker {
    pub fn new(mutation_strategy: MutationStrategy) -> Self {
        Self {
            known_modules: HashSet::new(),
            mutation_strategy,
            population_dir: "ai_population".to_string(),
        }
    }

    pub fn mutate_project(&mut self, project_dir: &Path) -> std::io::Result<()> {
        self.discover_modules(project_dir);

        for entry in WalkDir::new(project_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                match path.extension().and_then(|e| e.to_str()) {
                    Some("rs") if !self.is_mod_rs(path) => {
                        self.mutate_rust_file(path)?;
                    }
                    _ => continue,
                }
            } else if path.is_dir() && !self.has_mod_rs(path) {
                self.maybe_create_module(path)?;
            }
        }

        self.update_mod_files(project_dir)?;
        Ok(())
    }

    fn discover_modules(&mut self, dir: &Path) {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if let Some(name) = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .filter(|s| *s != "mod")
            {
                self.known_modules.insert(name.to_string());
            }
        }
    }

    fn mutate_rust_file(&mut self, path: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        let mut rng = rand::thread_rng();

        let mutated = if rng.gen_bool(0.3) {
            self.add_module_usage(&content)
        } else {
            self.mutation_strategy.mutate_code(&content, None)
        };

        fs::write(path, mutated)
    }

    fn maybe_create_module(&self, dir: &Path) -> std::io::Result<()> {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            let module_name = self.generate_module_name();
            let new_file = dir.join(format!("{}.rs", module_name));
            fs::write(&new_file, self.generate_module_content(&module_name))?;
        }
        Ok(())
    }

    fn update_mod_files(&mut self, dir: &Path) -> std::io::Result<()> {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let mut modules = Vec::new();
                for file in fs::read_dir(path)? {
                    if let Ok(file) = file {
                        let fpath = file.path();
                        if let Some(name) = fpath
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .filter(|s| *s != "mod")
                        {
                            if fpath.extension().and_then(|e| e.to_str()) == Some("rs") {
                                modules.push(name.to_string());
                                self.known_modules.insert(name.to_string());
                            }
                        }
                    }
                }

                if !modules.is_empty() {
                    let mod_path = path.join("mod.rs");
                    let content = generate_mod_content(&modules);
                    fs::write(mod_path, content)?;
                }
            }
        }
        Ok(())
    }

    fn is_mod_rs(&self, path: &Path) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map_or(false, |n| n == "mod.rs")
    }

    fn has_mod_rs(&self, dir: &Path) -> bool {
        dir.join("mod.rs").exists()
    }

    fn generate_module_name(&self) -> String {
        let mut rng = rand::thread_rng();
        let prefix = ["util", "core", "helper", "process", "data"];
        format!(
            "{}_{}",
            prefix[rng.gen_range(0..prefix.len())],
            rng.gen_range(0..999)
        )
    }

    fn generate_module_content(&self, name: &str) -> String {
        format!(
            "// Module généré automatiquement: {}\n\
             use std::{{fs, io}};\n\n\
             pub fn process() -> io::Result<()> {{\n\
             \tOk(())\n\
             }}\n",
            name
        )
    }

    pub fn add_module_usage(&self, content: &str) -> String {
        if let Some(module) = self.known_modules.iter().next() {
            if let Ok(funcs) = self.discover_public_functions(module) {
                let mut rng = rand::thread_rng();
                let mut lines = content.lines().map(|l| l.to_string()).collect::<Vec<_>>();

                lines.insert(0, format!("use crate::{};", module));

                if !funcs.is_empty() {
                    let num_calls = rng.gen_range(1..=3);
                    for _ in 0..num_calls {
                        if let Some(func) = funcs.choose(&mut rng) {
                            if let Some(pos) = self.find_insertion_point(&lines) {
                                let call = self.generate_function_call(module, func);
                                lines.insert(pos, call);
                            }
                        }
                    }
                }

                return lines.join("\n");
            }
            format!("use crate::{};\n{}", module, content)
        } else {
            content.to_string()
        }
    }

    fn find_insertion_point(&self, lines: &[String]) -> Option<usize> {
        let mut rng = rand::thread_rng();
        let valid_positions: Vec<_> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| {
                let l = line.trim();
                l.ends_with(';') || l.ends_with('}') || l.ends_with('{')
            })
            .map(|(i, _)| i)
            .collect();

        valid_positions.choose(&mut rng).copied()
    }

    fn generate_function_call(&self, module: &str, func: &str) -> String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => format!("    let _ = {}::{}();", module, func),
            1 => format!("    if let Ok(_) = {}::{}() {{ }}", module, func),
            _ => format!("    {}::{}().unwrap_or_default();", module, func),
        }
    }

    fn discover_public_functions(&self, module: &str) -> std::io::Result<Vec<String>> {
        let mut funcs = Vec::new();
        let module_path = Path::new(&self.population_dir).join(format!("{}.rs", module));

        if let Ok(content) = fs::read_to_string(&module_path) {
            for line in content.lines() {
                if line.trim().starts_with("pub fn ") {
                    if let Some(name) = extract_function_name(line) {
                        funcs.push(name);
                    }
                }
            }
        }

        Ok(funcs)
    }
}

fn generate_mod_content(modules: &[String]) -> String {
    let mut content = String::new();
    for module in modules {
        content.push_str(&format!("pub mod {};\n", module));
    }
    content
}

fn extract_function_name(line: &str) -> Option<String> {
    line.split("pub fn ")
        .nth(1)
        .and_then(|s| s.split('(').next())
        .map(|s| s.trim().to_string())
}
