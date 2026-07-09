use crate::brain::DecisionTreeLearner; // Ajouter cet import
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug)] // Supprimer Serialize et Deserialize
pub struct VersioningBrain {
    pub patterns: HashMap<String, PatternStats>,
    pub context_weights: HashMap<String, f64>,
    pub error_history: Vec<ErrorRecord>,
    pub success_patterns: Vec<SuccessPattern>,
    pub decision_features: Vec<String>,
    pub tree_learner: Option<DecisionTreeLearner>, // Add tree_learner field
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatternStats {
    pub signature: String,
    pub occurrences: u32,
    pub impact_predictions: HashMap<String, u32>, // "major"|"minor"|"patch" => count
    pub success_rate: f64,
    pub last_contexts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRecord {
    pub pattern: String,
    pub predicted_impact: String,
    pub actual_impact: String,
    pub context: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessPattern {
    pub pattern: String,
    pub context: String,
    pub confidence: f64,
}

impl PatternStats {
    pub fn get_most_likely_impact(&self) -> String {
        self.impact_predictions
            .iter()
            .max_by_key(|(_, &count)| count)
            .map(|(impact, _)| impact.clone())
            .unwrap_or_else(|| "patch".to_string())
    }
}

impl VersioningBrain {
    pub fn extract_features(&self, content: &str, context: &str) -> Vec<f64> {
        vec![
            content.matches("pub").count() as f64,
            content.matches("trait").count() as f64,
            content.matches("struct").count() as f64,
            content.matches("impl").count() as f64,
            content.matches("fn").count() as f64,
            content.matches("breaking").count() as f64,
            content.matches("fix").count() as f64,
            (context.contains("api") || context.contains("public")) as i32 as f64,
            (context.contains("core") || context.contains("internal")) as i32 as f64,
        ]
    }

    pub fn predict_with_tree(&self, features: &[f64]) -> Option<String> {
        self.tree_learner.as_ref().and_then(|tree| tree.predict(features))
    }

    pub fn train_decision_tree(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(tree) = &mut self.tree_learner {
            tree.train()?;
        }
        Ok(())
    }

    pub fn load(path: &str) -> Self {
        // Charger depuis un fichier ou créer un nouveau
        Self {
            patterns: HashMap::new(),
            context_weights: HashMap::new(),
            error_history: Vec::new(),
            success_patterns: Vec::new(),
            decision_features: Vec::new(),
            tree_learner: None,
        }
    }
}

impl Default for VersioningBrain {
    fn default() -> Self {
        Self {
            patterns: HashMap::new(),
            context_weights: HashMap::new(),
            error_history: Vec::new(),
            success_patterns: Vec::new(),
            decision_features: Vec::new(),
            tree_learner: None,
        }
    }
}
