use std::process::Command;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use uuid::uuid7;

// Fonction de copie récursive (remplace copy_dir_all)
fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
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

#[derive(Deserialize)]
struct IaList {
    nature: Vec<String>,
    scientist: Vec<String>,
    life_form: Vec<String>,
}

fn init_ia_from_template(ia_type: &str) -> Option<(PathBuf, IaMetadata)> {
    let template_path = PathBuf::from("templates").join(ia_type);
    let uuid_short = uuid7().to_string();
    let target_path = PathBuf::from(ia_type).join(&uuid_short);

    copy_dir_recursive(&template_path, &target_path).ok()?;

    let metadata = IaMetadata {
        id: uuid_short.clone(),
        kind: ia_type.to_string(),
    };
    let metadata_str = ron::ser::to_string_pretty(&metadata, ron::ser::PrettyConfig::default()).ok()?;
    fs::write(target_path.join("metadata.ron"), metadata_str).ok()?;

    Some((target_path, metadata))
}

fn ensure_directories() {
    for dir in ["nature", "scientist", "environment", "laboratory"] {
        fs::create_dir_all(dir).unwrap();
    }
    fs::create_dir_all("laboratory/specimens").unwrap();
    fs::create_dir_all("laboratory/results").unwrap();
    fs::create_dir_all("environment/forms").unwrap();
    fs::create_dir_all("environment/fossils").unwrap();
}

fn check_existing_ias() -> (Vec<String>, Vec<String>) {
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

fn generate_initial_metadata() -> IaList {
    let (mut natures, mut scientists) = check_existing_ias();

    // Crée une nature si aucune n'existe
    if natures.is_empty() {
        if let Some((path, _)) = init_ia_from_template("nature") {
            natures.push(path.to_string_lossy().into_owned());
        }
    }

    // Crée un scientist si aucun n'existe
    if scientists.is_empty() {
        if let Some((path, _)) = init_ia_from_template("scientist") {
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
    fs::write("metadata.ron", metadata_str)
        .expect("Échec écriture metadata");

    list
}

#[derive(Serialize, Deserialize)]
struct GlobalMetadata {
    version: u32,
    last_run: String,
    total_runs: u32,
}

impl Default for GlobalMetadata {
    fn default() -> Self {
        Self {
            version: 1,
            last_run: chrono::Local::now().to_rfc3339(),
            total_runs: 0,
        }
    }
}

fn load_or_init_global_metadata() -> GlobalMetadata {
    if let Ok(data) = fs::read("metadata.bin") {
        bincode::deserialize(&data).unwrap_or_default()
    } else {
        let metadata = GlobalMetadata::default();
        save_global_metadata(&metadata);
        metadata
    }
}

fn save_global_metadata(metadata: &GlobalMetadata) {
    if let Ok(data) = bincode::serialize(metadata) {
        let _ = fs::write("metadata.bin", data);
    }
}

fn main() {
    ensure_directories();

    // Charge ou initialise le metadata global
    let mut global_meta = load_or_init_global_metadata();
    global_meta.total_runs += 1;
    global_meta.last_run = chrono::Local::now().to_rfc3339();
    save_global_metadata(&global_meta);

    // Charge ou génère metadata.ron
    let list: IaList = if Path::new("metadata.ron").exists() {
        ron::from_str(&fs::read_to_string("metadata.ron").unwrap()).unwrap()
    } else {
        generate_initial_metadata()
    };

    // Lance les IAs existantes
    for path in list.nature.iter().chain(&list.scientist).chain(&list.life_form) {
        Command::new("cargo")
            .args([
                "run", "--manifest-path",
                &format!("{}/Cargo.toml", path)
            ])
            .spawn()
            .unwrap();
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
