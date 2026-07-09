use crate::config::get_config_path;
use crate::config::save_config;
use crate::config::ProjectConfig;
use crate::paths::get_component_path;
use crate::paths::get_root_path;
use std::fs;

/// Crée si besoin le dossier du composant
pub fn ensure_component_path(name: &str) -> std::path::PathBuf {
    let path = get_component_path(name);
    fs::create_dir_all(&path).unwrap();
    path
}

/// Crée un fichier `project_config.ron` par défaut si absent
pub fn ensure_config_exists() -> std::io::Result<()> {
    let path = get_config_path();
    if !path.exists() {
        let default = ProjectConfig {
            name: "default_project".into(),
            mode: Some("ide".into()),
            multi_project: Some(false),
        };
        save_config(&default)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
    }
    Ok(())
}

/// Active automatiquement les composants nécessaires en fonction du mode
pub fn auto_activate_components() {
    if crate::context::is_ide_mode() {
        ensure_component_path("ide_web");
    }
    if crate::context::is_intelligent_mode() {
        ensure_component_path("intelligent_engine");
    }
}

/// Crée un fichier de log général pour le projet
pub fn create_project_log() {
    let root_path = crate::paths::get_root_path();
    let log_path = root_path
        .parent()
        .map(|p| p.join(".intelli/logs/init.md"))
        .unwrap_or_else(|| root_path.join(".intelli/logs/init.md"));

    if let Some(parent) = log_path.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }

    let workspace = crate::workspace::WorkspaceInfo::new();
    let log_content = format!(
        "# Projet lancé : {}\n\nChemin racine : {:?}\nComposants actifs : {:?}\n",
        workspace.name, workspace.root_path, workspace.components
    );

    std::fs::write(log_path, log_content).unwrap();
}

/// Initialise la structure du projet
pub fn init_project_structure() -> std::io::Result<()> {
    let root_path = get_root_path();
    if !root_path.exists() {
        std::fs::create_dir_all(&root_path)?;
        ensure_config_exists()?;
    }
    Ok(())
}

/// Crée un fichier README.md avec des informations sur le projet
pub fn create_readme() {
    // Créer un README dans .intelli/ ou .graphver/ mais pas à la racine du projet
    let root_path = crate::paths::get_root_path();

    // Vérifions si nous sommes déjà dans un sous-dossier spécial
    let is_already_subdir = root_path.to_string_lossy().contains(".intelli")
        || root_path.to_string_lossy().contains(".graphver")
        || root_path.to_string_lossy().contains(".ide");

    // Si on est déjà dans un sous-dossier, on y met le README, sinon on crée un sous-dossier dédié
    let readme_path = if is_already_subdir {
        root_path.join("README.md")
    } else {
        // Créer un sous-dossier .workspace_manager pour éviter d'écraser le README principal
        let subdir = root_path.join(".workspace_manager");
        std::fs::create_dir_all(&subdir).unwrap_or_default();
        subdir.join("README.md")
    };

    let content = format!(
        "# Projet : {}\n\nMode : {}\nMulti-projet : {}\n\n## Accès à l'interface\n\nL'interface web est disponible à l'adresse http://localhost:4000\n",
        crate::context::get_project_name(),
        crate::context::get_mode(),
        crate::context::is_multi_project()
    );

    let readme_display = format!("{}", readme_path.display());
    std::fs::write(&readme_path, content).unwrap_or_else(|e| {
        eprintln!("Erreur lors de la création du README : {}", e);
    });

    println!("✅ Documentation créée dans {}", readme_display);
    println!("🌐 Interface disponible sur http://localhost:4000");
}
