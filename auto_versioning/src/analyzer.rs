use crate::brain::types::{Impact, Context, Criticality};
use crate::brain::learner::BrainLearner;
use crate::graph::DependencyGraph;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct ChangeHistory {
    caused_issues: bool,
    patterns: Vec<String>,
    impact: String,
}

pub struct ImpactAnalyzer {
    graph: HashMap<String, Vec<String>>,
    history: Vec<ChangeHistory>,
    pattern_weights: HashMap<String, f64>,
}

impl ImpactAnalyzer {
    pub fn analyze_change(&mut self, file_path: &str, content: &str, brain: &mut BrainLearner) -> Impact {
        let context = self.extract_context(file_path);
        let criticality = self.detect_patterns(content);
        
        let impact = brain.analyze_change(content, &context.to_string());  // Utiliser analyze_change
        
        brain.record_prediction(content, &context.to_string(), &impact);  // Mettre à jour l'apprentissage
        
        impact
    }

    fn extract_context(&self, file_path: &str) -> Context {
        // Analyse du rôle du fichier dans le projet
        let is_public_api = file_path.contains("/api/") || file_path.contains("/public/");
        let is_core = file_path.contains("/core/") || file_path.contains("/internal/");
        // ...
        Context { is_public_api, is_core }
    }

    fn detect_patterns(&self, content: &str) -> Criticality {
        let breaking_changes = content.matches("breaking").count();
        let new_features = content.matches("feature").count();
        let fixes = content.matches("fix").count();
        // ...
        Criticality::from_counts(breaking_changes, new_features, fixes)
    }

    fn learn_from_history(&mut self) {
        let patterns_to_adjust: Vec<Vec<String>> = self.history
            .iter()
            .filter(|r| r.caused_issues)
            .map(|r| r.patterns.clone())
            .collect();

        for patterns in patterns_to_adjust {
            self.adjust_weights(&patterns);
        }
    }

    fn compute_final_impact(&self, context: Context, criticality: Criticality, 
                          prediction: Option<String>) -> Impact {
        let confidence = if context.is_public_api {
            0.9
        } else if context.is_core {
            0.8
        } else {
            0.6
        };

        let level = if criticality.breaking_changes > 0 {
            "major".to_string()
        } else if criticality.new_features > 0 {
            "minor".to_string()
        } else {
            "patch".to_string()
        };

        Impact { level, confidence }
    }

    fn adjust_weights(&mut self, patterns: &[String]) {
        // Implémentation de l'ajustement des poids
        for pattern in patterns {
            self.pattern_weights.insert(pattern.clone(), 1.0);
        }
    }

    fn get_pattern_weights(&self) -> HashMap<String, String> {
        self.pattern_weights
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect()
    }

    pub fn analyze_changes(&mut self, paths: &[&Path]) -> Vec<String> {
        let mut impacts = Vec::new();
        let weights = self.get_pattern_weights();
        for path in paths {
            if let Some(impact) = determine_file_impact(path, &weights) {
                impacts.push(impact);
            }
        }
        impacts
    }
}

pub fn determine_file_impact(path: &Path, weights: &HashMap<String, String>) -> Option<String> {
    if let Ok(content) = fs::read_to_string(path) {
        for (pattern, impact) in weights {
            if content.contains(pattern) {
                return Some(impact.clone());
            }
        }
        Some("patch".to_string())  // default
    } else {
        None
    }
}

pub fn scan_directory(dir: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(scan_directory(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path);
            }
        }
    }
    files
}

pub fn scan_directory_for_rules(dir: &PathBuf) -> Vec<BumpRule> {
    let mut rules = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                rules.extend(scan_directory_for_rules(&path));
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                if let Some(rel_path) = path.strip_prefix(dir).ok().and_then(|p| p.to_str()) {
                    // Créer un HashMap par défaut pour les poids
                    let default_weights: HashMap<String, String> = HashMap::new();
                    if let Some(impact) = determine_file_impact(&path, &default_weights) {
                        rules.push(BumpRule {
                            path: rel_path.to_string(),
                            impact,
                        });
                    }
                }
            }
        }
    }
    rules
}

pub fn determine_bump(impacts: &[String]) -> Option<String> {
    if impacts.contains(&"major".to_string()) {
        Some("major".to_string())
    } else if impacts.contains(&"minor".to_string()) {
        Some("minor".to_string())
    } else if impacts.contains(&"patch".to_string()) {
        Some("patch".to_string())
    } else {
        None
    }
}

#[derive(Debug)]
pub struct BumpRule {
    pub path: String,
    pub impact: String,
}