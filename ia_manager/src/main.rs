use std::fs;
use std::path::{Path, PathBuf};
use std::env;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
struct IaConfig {
    name: String,
    memory_path: String,
    sandbox_path: String,
    manifest_path: String,
    args: Vec<String>,
}

fn main() {
    println!("=== IA Manager - Générateur de configuration ===");
    
    // Trouver la racine du workspace
    let workspace_root = find_workspace_root();
    println!("Racine du workspace: {}", workspace_root.display());
    
    // Scan simple des projets d'IA existants
    let ias = scan_projects(&workspace_root);
    
    if ias.is_empty() {
        println!("Aucune IA détectée. Création de la configuration par défaut.");
    } else {
        println!("{} IA(s) détectée(s)", ias.len());
    }
    
    // Générer le fichier RON dans shared_center
    save_ron_config(&workspace_root, &ias);

    // Générer le fichier BIN dans shared_center
    save_bin_config(&workspace_root, &ias);

    // Créer les répertoires nécessaires et initialiser les fichiers binaires
    create_ia_environment(&workspace_root, &ias);
    
    println!("Configuration et environnement préparés pour toutes les IAs.");
}

fn find_workspace_root() -> PathBuf {
    // Commencer par le répertoire courant
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Chercher le Cargo.toml du workspace
    let mut path = current_dir.clone();
    loop {
        let cargo_path = path.join("Cargo.toml");
        if cargo_path.exists() && fs::read_to_string(&cargo_path).unwrap_or_default().contains("[workspace]") {
            return path;
        }
        
        if !path.pop() {
            break;
        }
    }
    
    // Si on ne trouve pas, utiliser le répertoire courant
    println!("⚠️ Workspace non trouvé, utilisation du répertoire courant");
    current_dir
}

fn scan_projects(root: &Path) -> Vec<IaConfig> {
    let mut ias = Vec::new();
    
    // Regarder dans le répertoire ai/
    let ai_dir = root.join("ai");
    if ai_dir.exists() && ai_dir.is_dir() {
        for entry in fs::read_dir(&ai_dir).unwrap().filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                let ia_name = path.file_name().unwrap().to_string_lossy().to_string();
                let manifest_path = format!("ai/{}/Cargo.toml", ia_name);
                
                if path.join("Cargo.toml").exists() {
                    println!("IA trouvée: {}", ia_name);
                    ias.push(IaConfig {
                        name: ia_name.clone(),
                        memory_path: format!("ai/{}/src/memory/storage", ia_name), // Correction du chemin
                        sandbox_path: format!("sandbox/{}", ia_name),
                        manifest_path,
                        args: vec!["--verbose".to_string()],
                    });
                }
            }
        }
    }
    
    // Si aucune IA trouvée, ajouter mother_ai par défaut
    if ias.is_empty() {
        ias.push(IaConfig {
            name: "mother_ai".to_string(),
            memory_path: "ai/mother_ai/src/memory/storage".to_string(), // Correction du chemin
            sandbox_path: "sandbox/mother_ai".to_string(),
            manifest_path: "ai/mother_ai/Cargo.toml".to_string(),
            args: vec!["--verbose".to_string()],
        });
    }
    
    ias
}

fn save_ron_config(workspace_root: &Path, ias: &[IaConfig]) {
    let config_path = workspace_root.join("shared_center").join("ia_list.ron");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap_or_else(|e| {
        panic!("Impossible de créer le répertoire: {}", e);
    });
    
    let pretty_config = ron::ser::PrettyConfig::new()
        .enumerate_arrays(true)
        .separate_tuple_members(true);
        
    let ron_data = ron::ser::to_string_pretty(&ias, pretty_config)
        .expect("Erreur lors de la sérialisation RON");
        
    fs::write(&config_path, ron_data).expect("Erreur lors de l'écriture du fichier RON");
    println!("✅ Fichier de configuration créé: {}", config_path.display());
}

fn save_bin_config(workspace_root: &Path, ias: &[IaConfig]) {
    let config_path = workspace_root.join("shared_center").join("ia_list.bin");
    fs::create_dir_all(config_path.parent().unwrap()).unwrap_or_else(|e| {
        panic!("Impossible de créer le répertoire: {}", e);
    });

    let bin_data = bincode_next::encode_to_vec(&ias, bincode_next::config::standard()).expect("Erreur lors de la sérialisation binaire");
    fs::write(&config_path, bin_data).expect("Erreur lors de l'écriture du fichier binaire");
    println!("✅ Fichier binaire de configuration créé: {}", config_path.display());
}

fn create_ia_environment(workspace_root: &Path, ias: &[IaConfig]) {
    println!("Création des environnements pour chaque IA...");
    
    for ia in ias {
        // Créer uniquement le répertoire sandbox
        let sandbox_dir = workspace_root.join(&ia.sandbox_path);
        
        fs::create_dir_all(&sandbox_dir).unwrap_or_else(|e| {
            println!("Erreur lors de la création du répertoire {}: {}", sandbox_dir.display(), e);
        });

        println!("✅ Répertoire sandbox créé pour {}: {}", ia.name, sandbox_dir.display());
    }
}
