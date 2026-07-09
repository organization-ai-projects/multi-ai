use crate::graph_memory::AiGraph;
use serde_json::Value;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Impact {
    Patch,
    Minor,
    Major,
}

pub fn classify_impact(files: &[String]) -> Impact {
    if files.iter().any(|f| f.contains("lib.rs") || f.contains("api")) {
        Impact::Major
    } else if files.iter().any(|f| f.ends_with(".md") || f.contains("README")) {
        Impact::Patch
    } else {
        Impact::Minor
    }
}

pub fn determine_best_strategy(history: &Value, graph: &AiGraph) -> String {
    let history_pattern = crate::analyzer::analyze_version_pattern(history, graph);

    let mut strategy = String::new();
    strategy.push_str("Basé sur l'historique : ");
    strategy.push_str(&history_pattern);
    strategy.push_str("\nRecommandations IA :\n");

    for pattern in graph.analyze_memory() {
        strategy.push_str(&format!("- {}\n", pattern));
    }

    strategy
}
