use super::parser_types::{ParseMetadata, ParseResult};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct ParserManager {
    cache: HashMap<String, ParseResult>,
}

impl ParserManager {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn parse_directory(&mut self, dir: &str) -> Vec<ParseResult> {
        let mut results = Vec::new();

        for entry in WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            if let Some(result) = self.parse_file(entry.path()) {
                results.push(result);
            }
        }

        results
    }

    pub fn parse_file(&mut self, path: &Path) -> Option<ParseResult> {
        let content = fs::read_to_string(path).ok()?;
        let ast = syn::parse_file(&content).ok();

        let metadata = ParseMetadata {
            functions_count: self.count_functions(&ast),
            imports_count: self.count_imports(&ast),
            loc: content.lines().count(),
        };

        Some(ParseResult {
            code: content,
            path: path.to_string_lossy().to_string(),
            ast,
            metadata,
        })
    }

    fn count_functions(&self, ast: &Option<syn::File>) -> usize {
        let mut count = 0;
        if let Some(file) = ast {
            for item in &file.items {
                if let syn::Item::Fn(_) = item {
                    count += 1;
                }
            }
        }
        count
    }

    fn count_imports(&self, ast: &Option<syn::File>) -> usize {
        let mut count = 0;
        if let Some(file) = ast {
            for item in &file.items {
                if let syn::Item::Use(_) = item {
                    count += 1;
                }
            }
        }
        count
    }
}
