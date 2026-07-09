use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use crate::models::{IaList, IaMetadata};
use ron;
use log::{info, error};

pub fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dst_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_recursive(&entry.path(), &dst_path)?;
        } else {
            fs::copy(entry.path(), dst_path)?;
        }
    }
    Ok(())
}

pub fn ensure_directories(project_base: &Path) {
    let dirs = [
        "nature", "scientist", "environment", "laboratory",
        "laboratory/specimens", "laboratory/results",
        "environment/forms", "environment/fossils"
    ];

    for dir in dirs {
        let path = project_base.join(dir);
        if let Err(e) = fs::create_dir_all(&path) {
            error!("Erreur création dossier {}: {}", path.display(), e);
        } else {
            info!("Dossier créé: {}", path.display());
        }
    }
}

pub fn init_ia_from_template(project_base: &Path, ia_type: &str) -> Option<(PathBuf, IaMetadata)> {
    let template_path = project_base.join("templates").join(ia_type);
    let uuid_short = Uuid::now_v7().to_string();
    let target_path = project_base.join(ia_type).join(&uuid_short);

    info!("Copie template {} vers {}", template_path.display(), target_path.display());
    
    match copy_dir_recursive(&template_path, &target_path) {
        Ok(_) => {
            info!("Template copié avec succès");
            let metadata = IaMetadata {
                id: uuid_short.clone(),
                kind: ia_type.to_string(),
            };
            let metadata_str = ron::ser::to_string_pretty(&metadata, ron::ser::PrettyConfig::default()).ok()?;
            fs::write(target_path.join("metadata.ron"), metadata_str).ok()?;

            Some((target_path, metadata))
        }
        Err(e) => {
            error!("Erreur copie template: {}", e);
            None
        }
    }
}

pub fn check_existing_ias() -> (Vec<String>, Vec<String>) {
    let mut natures = Vec::new();
    let mut scientists = Vec::new();
    
    // Vérifie les IAs nature existantes
    if let Ok(entries) = fs::read_dir("nature") {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                natures.push(entry.path().to_string_lossy().into_owned());
            }
        }
    }

    // Vérifie les IAs scientist existantes
    if let Ok(entries) = fs::read_dir("scientist") {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                scientists.push(entry.path().to_string_lossy().into_owned());
            }
        }
    }

    (natures, scientists)
}

pub fn generate_initial_metadata(project_base: &Path) -> IaList {
    let (mut natures, mut scientists) = check_existing_ias();

    // Crée une nature si aucune n'existe
    if natures.is_empty() {
        if let Some((path, _)) = init_ia_from_template(project_base, "nature") {
            natures.push(path.to_string_lossy().into_owned());
        }
    }

    // Crée un scientist si aucun n'existe
    if scientists.is_empty() {
        if let Some((path, _)) = init_ia_from_template(project_base, "scientist") {
            scientists.push(path.to_string_lossy().into_owned());
        }
    }

    let list = IaList {
        nature: natures,
        scientist: scientists,
        life_form: vec![],
    };

    // Sauvegarde du metadata.ron
    let metadata_str = ron::ser::to_string_pretty(&list, ron::ser::PrettyConfig::default())
        .expect("Échec sérialisation metadata");
    fs::write(project_base.join("metadata.ron"), metadata_str)
        .expect("Échec écriture metadata");

    list
}
