use super::{compilation::CompilationManager, mutation::MutationManager, parser::ParserManager};
use std::path::PathBuf;

pub struct CodeManager {
    parser: ParserManager,
    mutation: MutationManager,
    compilation: CompilationManager,
}

impl CodeManager {
    pub fn new() -> Self {
        Self {
            parser: ParserManager::new(),
            mutation: MutationManager::new(),
            compilation: CompilationManager::new(),
        }
    }

    // Les méthodes retournent juste les résultats, sans gestion de mémoire
    pub fn mutate(&mut self, code: &str) -> Option<String> {
        self.mutation.apply_mutation(code) // Juste la mutation
    }

    pub fn parse(&mut self, path: &str) -> Option<String> {
        self.parser.read_file(path)
    }

    pub fn compile(&mut self, code: &str, project_dir: &str) -> bool {
        let path = PathBuf::from(project_dir);
        self.compilation.compile(code, &path)
    }

    pub fn mutate_code(&mut self, code: &str, strategy: &str) -> Option<String> {
        // Juste la mutation technique, sans notion de cerveau/mémoire
        self.mutation.apply_mutation(code, strategy)
    }
}
