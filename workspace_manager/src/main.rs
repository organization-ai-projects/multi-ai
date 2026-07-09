use log::{error, info};
use std::fs;
use std::path::PathBuf;
use workspace_manager::{
    config::get_config_path,
    context::{get_mode, get_project_name, is_multi_project},
    http::start_http_server, // Import corrigé
    init::{auto_activate_components, create_readme, init_project_structure},
    multi::scan_sub_projects,
};

#[tokio::main]
async fn main() {
    // Initialiser le logger avec un niveau par défaut
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("🧠 Initialisation du projet intelligent...");

    // Étape 1 : création du dossier + fichier config si nécessaire
    if let Err(e) = init_project_structure() {
        error!("❌ Erreur d'initialisation : {}", e);
        return;
    }

    // Afficher le chemin du fichier de configuration
    let config_path = get_config_path();
    info!(
        "📄 Chemin du fichier de configuration : {}",
        config_path.display()
    );

    // Étape 2 : activer les composants selon le mode
    auto_activate_components();

    // Étape 3 : contexte du projet
    let name = get_project_name();
    let mode = get_mode();
    let multi = is_multi_project();

    info!("📦 Projet : {name}");
    info!("🔧 Mode : {mode}");
    info!("🔁 Multi-projet : {multi}");

    // Étape 4 : scan des sous-projets si activé
    if multi {
        info!("🔍 Scan des sous-projets...");
        let current = std::env::current_dir().unwrap();
        let projects = scan_sub_projects(&current);

        for p in &projects {
            info!("📁 [{}] {}", p.container_type, p.root_path.display());
        }

        // Étape 5 : sauvegarde JSON dans .intelli/
        let json_path = PathBuf::from(".intelli").join("projects.json");
        if let Some(parent) = json_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }

        let data = serde_json::to_string_pretty(&projects).unwrap();
        fs::write(&json_path, data).unwrap(); // Emprunter json_path ici

        info!("✅ Sous-projets enregistrés dans {}", json_path.display()); // json_path reste utilisable
    }

    // Étape 6 : Créer un fichier README.md
    create_readme();
    info!("📄 Fichier README.md créé !");

    // Démarrer le serveur HTTP
    let http_server = start_http_server();

    // Affichage clair de l'URL de l'interface
    println!("\n🚀 Démarrage du Workspace Manager");
    println!("📋 Services disponibles sur http://localhost:4000");
    println!("💻 Interface web accessible à cette adresse");
    println!("📌 Utilisez Ctrl+C pour arrêter le serveur\n");

    // Démarrer d'autres tâches si nécessaire
    http_server.await; // Correction de l'utilisation de join!

    info!("🚀 Projet prêt !");
}
