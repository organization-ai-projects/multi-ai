use crate::classifier::Impact;
use crate::graph_memory::AiGraph;

#[derive(Debug)]
pub struct SnapshotAnalysis {
    pub files_changed: Vec<String>,
    pub dominant_area: String,
    pub impact: Impact,
}

pub fn analyze_snapshot(files: &[String]) -> SnapshotAnalysis {
    let dominant_area = if files.iter().any(|f| f.contains("api") || f.contains("lib")) {
        "core".to_string()
    } else if files.iter().any(|f| f.contains("README") || f.contains("doc")) {
        "documentation".to_string()
    } else {
        "misc".to_string()
    };

    let impact = if dominant_area == "core" {
        Impact::Major
    } else if dominant_area == "documentation" {
        Impact::Patch
    } else {
        Impact::Minor
    };

    SnapshotAnalysis {
        files_changed: files.to_vec(),
        dominant_area,
        impact,
    }
}

pub fn analyze_version_pattern(history: &serde_json::Value, graph: &AiGraph) -> String {
    let ai_patterns = graph.analyze_memory();

    if let Some(nodes) = history.get("nodes").and_then(|n| n.as_array()) {
        let mut major_count = 0;
        let mut minor_count = 0;
        let mut patch_count = 0;

        for node in nodes {
            if let Some(impact) = node.get("impact").and_then(|i| i.as_str()) {
                match impact {
                    "Major" => major_count += 1,
                    "Minor" => minor_count += 1,
                    "Patch" => patch_count += 1,
                    _ => {}
                }
            }
        }

        let total = major_count + minor_count + patch_count;
        if total == 0 {
            return "Aucun changement détecté dans l'historique.".to_string();
        }

        let mut analysis = format!(
            "Historique des versions :\n- Major: {} ({}%)\n- Minor: {} ({}%)\n- Patch: {} ({}%)\n",
            major_count,
            (major_count as f32 / total as f32 * 100.0) as u8,
            minor_count,
            (minor_count as f32 / total as f32 * 100.0) as u8,
            patch_count,
            (patch_count as f32 / total as f32 * 100.0) as u8
        );

        if !ai_patterns.is_empty() {
            analysis.push_str("\n🔮 Prédictions IA :\n");
            for pattern in ai_patterns {
                analysis.push_str(&format!("- {}\n", pattern));
            }
        }

        analysis
    } else {
        "Aucun pattern détecté dans l'historique.".to_string()
    }
}
