//utilise les fichiers de capacities/ pour les orchestrer et exposer
// les fichiers de capacités il est le seul à pouvoir être utilisé à l'extérieur

use super::code::CodeManager;
use super::nlp::NlpManager;
use crate::capacities::mutate_ast::{mutate_randomly, MutationResult, Strategy};
use crate::capacities::parser::{extract_all_snippets, extract_all_snippets_paginated};
use crate::capacities::try_compilation::compile_snippet;
use uuid::Uuid;

pub struct CapacitiesAPI {
    code_manager: CodeManager,
    nlp_manager: NlpManager,
}

impl CapacitiesAPI {
    pub fn new() -> Self {
        Self {
            code_manager: CodeManager::new(),
            nlp_manager: NlpManager::new(),
        }
    }

    pub fn discover_code_paths(root: &str, patterns: &[&str]) -> Vec<String> {
        let mut paths = Vec::new();
        for pattern in patterns {
            paths.extend(
                glob::glob(&format!("{}/{}", root, pattern))
                    .unwrap()
                    .filter_map(Result::ok)
                    .map(|p| p.to_string_lossy().to_string()),
            );
        }
        paths
    }

    /// Extrait tous les snippets de code d'un répertoire donné
    pub fn extract_snippets(&self, dir: &str, limit: Option<usize>) -> Vec<(String, String)> {
        self.code_manager.parse_code(dir)
    }

    /// Compile un snippet de code et retourne le résultat
    pub fn compile_snippet(snippet: &str, base_dir: &str) -> (bool, i32, Option<String>) {
        compile_snippet(snippet, base_dir)
    }

    /// Applique une mutation aléatoire sur un code donné
    pub fn mutate_code_randomly(&self, code: &str) -> Option<String> {
        self.code_manager.mutate_code(code, "random")
    }

    /// Applique une mutation spécifique sur un code donné
    pub fn mutate_code_with_strategy(
        code: &str,
        strategy: Strategy,
        context: Option<&str>,
    ) -> Option<MutationResult> {
        mutate_with_strategy(code, strategy, context)
    }
}
