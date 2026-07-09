use crate::bridge::smart_bridge::{suggest_version_strategy, smart_version_bump};

pub fn auto_comment_function(file_content: &str) -> Result<String, String> {
    if let Some(strategy) = suggest_version_strategy() {
        if strategy.get("impact") == Some(&serde_json::json!("Major")) {
            return Ok("// ⚠️ Modification majeure détectée\n".to_string() + file_content);
        }
    }

    if file_content.is_empty() {
        return Err("Le contenu du fichier est vide.".to_string());
    }

    let lines: Vec<&str> = file_content.lines().collect();
    let mut result = String::new();

    for line in lines {
        if line.trim_start().starts_with("fn ") {
            result.push_str("// TODO: Ajouter un commentaire pour cette fonction\n");
        }
        result.push_str(line);
        result.push('\n');
    }

    // Ajouter une suggestion de bump de version
    if let Ok(bump_result) = smart_version_bump() {
        result.push_str(&format!("\n// Suggestion de bump : {}\n", bump_result));
    }

    Ok(result)
}

pub fn auto_format_code(file_content: &str) -> Result<String, String> {
    if file_content.is_empty() {
        return Err("Le contenu du fichier est vide.".to_string());
    }

    let formatted = file_content
        .lines()
        .map(|line| format!("    {}", line.trim()))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(formatted)
}

pub fn summarize_file(file_content: &str) -> Result<String, String> {
    if file_content.is_empty() {
        return Err("Le contenu du fichier est vide.".to_string());
    }

    let line_count = file_content.lines().count();
    let function_count = file_content
        .lines()
        .filter(|line| line.trim_start().starts_with("fn "))
        .count();

    Ok(format!(
        "Résumé :\n- Nombre de lignes : {}\n- Nombre de fonctions : {}",
        line_count, function_count
    ))
}
