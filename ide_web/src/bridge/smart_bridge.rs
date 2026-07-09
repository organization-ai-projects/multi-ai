use super::{ai_bridge, version_bridge};
use serde_json::Value;

/// Effectue un bump de version intelligent en combinant IA et versioning
pub async fn smart_version_bump() -> Result<String, String> {
    let files = version_bridge::send_version_command("get_modified_files").await?;
    let impact_analysis = ai_bridge::analyze_changes(&serde_json::from_str(&files).unwrap()).await?;
    let prediction = ai_bridge::send_ai_command("predict_context").await?;
    version_bridge::send_version_command(&format!("bump_with_context:{}:{}", impact_analysis, prediction)).await
}

/// Propose une stratégie de version basée sur l'historique et l'IA
pub async fn suggest_version_strategy() -> Result<Value, String> {
    let version_history = version_bridge::get_version_graph().await?;
    let version_pattern = ai_bridge::analyze_version_pattern(&serde_json::from_str(&version_history).unwrap()).await?;
    let combined_strategy = ai_bridge::determine_best_strategy(&serde_json::from_str(&version_history).unwrap()).await?;

    Ok(serde_json::json!({
        "version_pattern": version_pattern,
        "combined_strategy": combined_strategy
    }))
}
