use crate::config::{load_config, ProjectConfig};

/// Retourne toute la config ou un fallback vide
pub fn get_config() -> ProjectConfig {
    load_config().unwrap_or(ProjectConfig {
        name: "default_project".into(),
        mode: None,
        multi_project: None,
    })
}

/// Retourne le nom du projet
pub fn get_project_name() -> String {
    get_config().name
}

/// Retourne le mode (ide, intelligent, version-only, etc.)
pub fn get_mode() -> String {
    get_config().mode.unwrap_or_else(|| "unknown".to_string())
}

/// true si le mode est exactement "ide"
pub fn is_ide_mode() -> bool {
    get_mode() == "ide"
}

/// true si le mode est "intelligent"
pub fn is_intelligent_mode() -> bool {
    get_mode() == "intelligent"
}

/// true si le mode est "version-only"
pub fn is_version_only_mode() -> bool {
    get_mode() == "version-only"
}

/// true si config multi_projet = true
pub fn is_multi_project() -> bool {
    get_config().multi_project.unwrap_or(false)
}
