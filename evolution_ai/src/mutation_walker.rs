use crate::mutations::MutationStrategy; // Updated import
use rand::Rng;
use std::{collections::HashSet, fs, path::Path};
use walkdir::WalkDir;

pub struct MutationWalker {
    known_modules: HashSet<String>,
    mutation_strategy: MutationStrategy,
}

impl MutationWalker {
    pub fn new(mutation_strategy: MutationStrategy) -> Self {
        Self {
            known_modules: HashSet::new(),
            mutation_strategy,
        }
    }

    pub fn mutate_project(&mut self, project_dir: &Path) -> std::io::Result<()> {
        // Découverte des modules
        self.discover_modules(project_dir);

        // Mutation récursive
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
                // Potentiellement créer un nouveau module
                self.maybe_create_module(path)?;
            }
        }

        // Mise à jour des mod.rs
        self.update_mod_files(project_dir)?;
        Ok(())
    }

    fn discover_modules(&mut self, dir: &Path) {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if let Some(name) = entry
                .path()
                .file_stem()
                .and_then(|s| s.to_str())
                .filter(|s| s != "mod")
            {
                self.known_modules.insert(name.to_string());
            }
        }
    }

    fn maybe_create_module(&self, dir: &Path) -> std::io::Result<()> {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.1) {
            // 10% de chance de créer un nouveau module
            let module_name = self.generate_module_name();
            let new_file = dir.join(format!("{}.rs", module_name));
            fs::write(&new_file, self.generate_module_content(&module_name))?;
        }
        Ok(())
    }

    fn mutate_rust_file(&self, path: &Path) -> std::io::Result<()> {
        let content = fs::read_to_string(path)?;
        let mut rng = rand::thread_rng();

        // 30% de chance d'ajouter un use d'un autre module
        if rng.gen_bool(0.3) {
            let mutated = self.add_module_usage(&content);
            fs::write(path, mutated)?;
        } else {
            // Mutation normale du code
            let mutated = self.mutation_strategy.mutate_code(&content, path);
            fs::write(path, mutated)?;
        }
        Ok(())
    }

    fn update_mod_files(&self, dir: &Path) -> std::io::Result<()> {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let mod_file = path.join("mod.rs");
                if mod_file.exists() {
                    self.update_single_mod_file(&mod_file)?;
                }
            }
        }
        Ok(())
    }

    fn update_single_mod_file(&self, mod_path: &Path) -> std::io::Result<()> {
        let dir = mod_path.parent().unwrap();
        let mut modules = Vec::new();

        // Collecter tous les fichiers .rs du dossier
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(name) = path
                .file_stem()
                .and_then(|s| s.to_str())
                .filter(|s| s != "mod")
            {
                if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                    modules.push(name.to_string());
                }
            }
        }

        // Générer le contenu du mod.rs
        let content = modules
            .iter()
            .map(|name| format!("pub mod {};", name))
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(mod_path, content)
    }

    // Helper methods...
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

    fn add_module_usage(&self, content: &str) -> String {
        if let Some(module) = self.known_modules.iter().next() {
            // Trouver les fonctions publiques disponibles dans le module cible
            if let Ok(funcs) = self.discover_public_functions(module) {
                let mut rng = rand::thread_rng();
                let mut lines: Vec<_> = content.lines().collect();

                // Ajouter le use
                lines.insert(0, &format!("use crate::{};", module));

                // Insérer des appels aux fonctions à des positions aléatoires
                if !funcs.is_empty() {
                    // Nombre d'appels à insérer (1 à 3)
                    let num_calls = rng.gen_range(1..=3);
                    for _ in 0..num_calls {
                        let func = funcs.choose(&mut rng).unwrap();
                        let call = self.generate_function_call(module, func);

                        // Trouver un endroit approprié pour insérer l'appel
                        if let Some(pos) = self.find_insertion_point(&lines, &mut rng) {
                            lines.insert(pos, &call);
                        }
                    }
                }

                lines.join("\n")
            } else {
                // Fallback si on ne peut pas lire le module
                format!("use crate::{};\n{}", module, content)
            }
        } else {
            content.to_string()
        }
    }

    fn discover_public_functions(&self, module: &str) -> std::io::Result<Vec<String>> {
        let mut funcs = Vec::new();
        // Chercher le fichier du module
        let module_path = Path::new("src").join(format!("{}.rs", module));

        if let Ok(content) = fs::read_to_string(&module_path) {
            // Parser basique pour trouver les fonctions publiques
            for line in content.lines() {
                let line = line.trim();
                if line.starts_with("pub fn ") {
                    if let Some(func_name) = line
                        .split("pub fn ")
                        .nth(1)
                        .and_then(|s| s.split('(').next())
                        .map(|s| s.trim().to_string())
                    {
                        funcs.push(func_name);
                    }
                }
            }
        }
        Ok(funcs)
    }

    fn generate_function_call(&self, module: &str, func: &str) -> String {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => format!("    let _ = {}::{}();", module, func),
            1 => format!("    if let Ok(_) = {}::{}() {{ }}", module, func),
            _ => format!("    {}::{}().unwrap_or_default();", module, func),
        }
    }

    fn find_insertion_point<'a>(
        &self,
        lines: &[&'a str],
        rng: &mut dyn rand::RngCore,
    ) -> Option<usize> {
        let valid_positions: Vec<_> = lines
            .iter()
            .enumerate()
            .filter(|(_, line)| {
                let l = line.trim();
                l.ends_with(';') || l.ends_with('}') || l.ends_with('{')
            })
            .map(|(i, _)| i)
            .collect();

        if valid_positions.is_empty() {
            None
        } else {
            Some(valid_positions[rng.gen_range(0..valid_positions.len())])
        }
    }
}
