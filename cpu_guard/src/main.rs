mod config;
mod monitoring; // Ce module fait maintenant référence au dossier monitoring/ et son fichier mod.rs
mod process_control;
mod logging;

use std::thread;
use std::fs;

fn main() {
    // Préparation du répertoire de logs - chemin depuis la racine du workspace
    let log_dir = "shared_center/monitoring_logs";
    fs::create_dir_all(log_dir).expect("Impossible de créer le répertoire de logs");
    
    // Charger la configuration des IA - chemin depuis la racine du workspace
    let config_path = "shared_center/ia_list.ron";
    let ia_list = config::load_ia_config(config_path)
        .expect("Impossible de charger la configuration des IA");
    
    // Créer le logger
    let logger = logging::create_session_logger(log_dir);
    let (bin_path, ron_path) = logger.file_paths();
    println!("📝 Logs enregistrés dans :");
    println!("   - BIN (pour IA) : {}", bin_path);
    println!("   - RON (pour humain) : {}", ron_path);
    
    // Version parallèle avec threads
    let mut handles = vec![];
    
    for ia in ia_list {
        // Clone nécessaire pour le move dans le thread
        let logger_clone = logger.clone();
        
        // Lancement d'un thread dédié pour cette IA
        let handle = thread::spawn(move || {
            monitoring::monitor_ia(ia, logger_clone); // On utilise l'export depuis monitoring/mod.rs
        });
        
        handles.push(handle);
    }
    
    // Attendre que tous les threads de surveillance terminent
    for handle in handles {
        handle.join().unwrap();
    }
}
