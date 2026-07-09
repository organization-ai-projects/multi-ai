use crate::bridge::smart_bridge::{smart_version_bump, revert_visual, suggest_version_strategy};

pub fn get_suggestion() -> String {
    if let Some(strategy) = suggest_version_strategy() {
        strategy.to_string()
    } else {
        "Pas de suggestion disponible".into()
    }
}

pub fn bump_version() -> Result<(), String> {
    smart_version_bump().map_err(|e| e.to_string())
}

pub fn revert_to(id: &str) -> Result<(), String> {
    let result = revert_visual(id);
    if result.contains("↩️") {
        Ok(())
    } else {
        Err(result)
    }
}
