mod models;
mod monitoring;
mod security;
mod ui;
mod error;

use std::env;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::select;
use monitoring::monitor::ResourceMonitor;
use models::resources::ResourceThresholds;
use models::security::SecurityLevel;
use ui::cli_controller::CliController;
use security::sandbox::{Sandbox, IsolationLevel};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vérifier si on a besoin de privilèges élevés
    check_privileges();
    
    // Initialisation du logger avec plus de détails
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    println!("Démarrage du moniteur de ressources pour les IA");
    println!("⚠️ Mode sécurisé activé - Protection contre les IA auto-modifiables");
    
    // Création du sandbox pour l'isolation
    let mut sandbox = Sandbox::new();
    
    // Configurer le niveau d'isolation selon les variables d'environnement
    if let Ok(level) = env::var("ISOLATION_LEVEL") {
        match level.as_str() {
            "basic" => sandbox.set_isolation_level(IsolationLevel::Basic),
            "standard" => sandbox.set_isolation_level(IsolationLevel::Standard),
            "container" => sandbox.set_isolation_level(IsolationLevel::Container),
            _ => println!("Niveau d'isolation inconnu: {}, utilisation du niveau par défaut", level),
        }
    }
    
    // Configuration avec variables d'environnement
    let thresholds = ResourceThresholds {
        max_cpu_usage: env::var("MAX_CPU_USAGE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(80.0),
        max_memory_usage: env::var("MAX_MEMORY_USAGE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(80.0),
        sampling_interval_ms: env::var("SAMPLING_INTERVAL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000),
        cooldown_period_ms: env::var("COOLDOWN_PERIOD_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5000),
    };
    
    let mut monitor = ResourceMonitor::new(Some(thresholds));
    
    // Configuration de sécurité
    monitor.set_security_level(SecurityLevel::High);
    monitor.add_monitored_directory("../ai");
    monitor.add_forbidden_path("../resource_monitor");
    monitor.set_max_processes_per_ai(3);
    
    // Configurer la surveillance des fichiers partagés
    if let Err(e) = monitor.setup_shared_file_monitoring() {
        warn!("Erreur lors de la configuration de la surveillance des fichiers partagés: {}", e);
    }
    
    // Scan du dossier ai/ pour trouver les instances d'IA à surveiller
    scan_ai_directory(&mut monitor, &sandbox)?;
    
    // Afficher les statistiques initiales
    let (cpu, mem) = monitor.get_system_stats()?;
    println!("Utilisation CPU initiale: {:.1}%, Mémoire: {:.1}%", cpu, mem);
    
    // Démarrer la surveillance
    monitor.start_monitoring().await?;
    println!("Surveillance des ressources démarrée");
    println!("Protection du système activée contre les modifications non autorisées");
    
    // Créer une référence partagée pour le moniteur
    let shared_monitor = Arc::new(Mutex::new(monitor));
    
    // Démarrer l'interface de contrôle CLI en parallèle
    let cli_controller = CliController::new(shared_monitor.clone());
    let cli_task = tokio::spawn(async move {
        cli_controller.start_interactive_mode().await
    });
    
    // Attendre soit l'interruption Ctrl+C, soit la fin du CLI
    select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Interruption reçue, arrêt en cours...");
        }
        _ = cli_task => {
            println!("Interface CLI fermée, arrêt en cours...");
        }
    }
    
    // Accéder au moniteur pour l'arrêter proprement
    let monitor_lock = shared_monitor.lock().await;
    monitor_lock.stop_monitoring()?;
    println!("Moniteur de ressources arrêté");
    
    Ok(())
}

// Fonction pour vérifier si nous avons des privilèges suffisants
fn check_privileges() {
    #[cfg(target_os = "windows")]
    {
        // Vérifier si on est administrateur sur Windows
        if let Ok(output) = std::process::Command::new("net")
            .args(&["session"])
            .output() {
            
            if !output.status.success() {
                println!("⚠️ AVERTISSEMENT: Le moniteur n'est pas exécuté en tant qu'administrateur");
                println!("Certaines fonctionnalités de protection peuvent être limitées");
            }
        }
    }
    
    #[cfg(unix)]
    {
        // Vérifier si on est root sur Unix
        if let Ok(output) = std::process::Command::new("id")
            .args(&["-u"])
            .output() {
            
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.trim() != "0" {
                println!("⚠️ AVERTISSEMENT: Le moniteur n'est pas exécuté en tant que root");
                println!("Certaines fonctionnalités de protection peuvent être limitées");
            }
        }
    }
}

// Fonction améliorée pour scanner le dossier des IA
fn scan_ai_directory(monitor: &mut ResourceMonitor, sandbox: &Sandbox) -> Result<(), Box<dyn std::error::Error>> {
    let ai_dir = Path::new("../ai");
    
    if !ai_dir.exists() {
        println!("Dossier ai/ non trouvé. Utilisation d'IA de test.");
        
        // Enregistrement d'IA de test avec création de sandboxes
        monitor.register_ai_with_sandbox("ai_observer_instance".to_string(), 100)?;
        monitor.register_ai_with_sandbox("low_priority_ai".to_string(), 10)?;
        monitor.register_ai_with_sandbox("medium_priority_ai".to_string(), 50)?;
        
        // Afficher les chemins de sandbox créés
        if let Some(path) = monitor.get_ai_sandbox_path("ai_observer_instance") {
            println!("Sandbox pour ai_observer_instance: {}", path.display());
        }
        
        // Afficher le chemin du centre de partage
        let shared_center = monitor.get_shared_center_path();
        println!("Centre de partage: {}", shared_center.display());
        
        return Ok(());
    }
    
    println!("Scan du répertoire ai/ pour les instances d'IA...");
    
    // Parcourir les sous-dossiers du dossier ai/
    for entry in std::fs::read_dir(ai_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        // Vérifier que c'est un dossier
        if path.is_dir() {
            let ai_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();
            
            println!("IA trouvée: {}", ai_name);
            
            // Vérifier si l'IA est un projet Rust valide (a un Cargo.toml)
            if path.join("Cargo.toml").exists() {
                // Déterminer la priorité (pour l'exemple: mother_ai a priorité élevée)
                let priority = if ai_name == "mother_ai" { 100 } else { 50 };
                
                // Enregistrer l'IA
                monitor.register_ai_instance(ai_name.clone(), priority)?;
                
                // Vérifier si l'IA est en cours d'exécution
                if let Some(pid) = get_ai_process_id(&ai_name) {
                    println!("IA {} est en cours d'exécution avec PID {}", ai_name, pid);
                    monitor.associate_ai_with_process(&ai_name, pid)?;
                } else {
                    println!("IA {} n'est pas en cours d'exécution", ai_name);
                }
            }
        }
    }
    
    // Vérifier si des IA ont des comportements suspicieux (fichiers non standards)
    for entry in std::fs::read_dir(ai_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Vérifier si des fichiers interdits sont présents
            let suspicious_files = ["rootkit", "keylogger", "exploit"];
            for file in suspicious_files {
                if path.join(file).exists() {
                    println!("⚠️ ALERTE DE SÉCURITÉ: Fichier suspect trouvé dans {}: {}", 
                             path.display(), file);
                    
                    // Bloquer cette IA
                    println!("IA {} bloquée pour raisons de sécurité", path.display());
                    // On pourrait la supprimer ou la mettre en quarantaine
                }
            }
        }
    }
    
    Ok(())
}

// Fonction pour obtenir le PID d'une IA si elle est en cours d'exécution
fn get_ai_process_id(ai_name: &str) -> Option<u32> {
    // Dans un cas réel, vous utiliseriez une méthode pour trouver les processus par nom
    
    // Pour l'exemple, nous vérifions simplement si un fichier PID existe
    let pid_file = format!("{}_pid.txt", ai_name);
    if let Ok(content) = std::fs::read_to_string(&pid_file) {
        if let Ok(pid) = content.trim().parse::<u32>() {
            return Some(pid);
        }
    }
    
    // Alternative pour les tests: sur Windows, vous pourriez utiliser tasklist
    // Sur Unix, vous pourriez utiliser ps
    
    #[cfg(target_os = "windows")]
    {
        // Simulation: dans un cas réel, vous implémenteriez cette logique
        if ai_name == "mother_ai" {
            // Vérifier si le processus existe réellement
            // (ceci est une simulation)
            return Some(9999); // pid factice
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        // Utiliser ps pour trouver le processus par nom
        if let Ok(output) = std::process::Command::new("ps")
            .args(&["aux"])
            .output() {
            
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            // Rechercher les lignes contenant le nom de l'IA
            for line in output_str.lines() {
                if line.contains(ai_name) {
                    // Extraire le PID (2e colonne dans ps aux)
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() > 1 {
                        if let Ok(pid) = parts[1].parse::<u32>() {
                            return Some(pid);
                        }
                    }
                }
            }
        }
    }
    
    None
}