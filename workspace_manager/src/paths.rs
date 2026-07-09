use std::path::{PathBuf};

/// Détecte la racine du projet intelligent (.ide, .intelli, .graphver, sinon .)
pub fn get_root_path() -> PathBuf {
    let root = std::env::current_dir().unwrap();
    if root.join(".ide").exists() {
        root.join(".ide")
    } else if root.join(".intelli").exists() {
        root.join(".intelli")
    } else if root.join(".graphver").exists() {
        root.join(".graphver")
    } else {
        root
    }
}

/// Construit le chemin vers un composant interne (ex: ai_assistant, logs, etc.)
pub fn get_component_path(name: &str) -> PathBuf {
    get_root_path().join(name)
}

/// Liste tous les projets dans le dossier global .intelli/
pub fn list_projects() -> Vec<String> {
    let root = get_root_path();
    let global_path = root.parent().unwrap_or(&root).join(".intelli");
    if global_path.exists() {
        std::fs::read_dir(global_path)
            .unwrap()
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().is_dir())
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect()
    } else {
        vec![]
    }
}

/// Retourne le chemin racine d'un projet spécifique
pub fn get_project_root(name: &str) -> PathBuf {
    let root = get_root_path();
    root.parent().unwrap_or(&root).join(".intelli").join(name)
}
