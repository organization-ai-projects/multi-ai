// src/api.rs

/// Liste toutes les IA connues
pub fn list_ais() -> Vec<String> {
    // Ex: Scan des dossiers, accès base de données, etc.
    vec!["copilot".into(), "super_agent".into()]
}

/// Crée une nouvelle IA, callable par CLI et par IA
pub fn create_ai(name: &str) -> Result<(), String> {
    // Logique de création d’IA (ici: simulation)
    if name.is_empty() {
        Err("Nom d’IA vide !".into())
    } else {
        println!("✅ Création IA: {}", name);
        Ok(())
    }
}
