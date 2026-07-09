use crate::config::{IaConfig, GuardianSettings};
use crate::logging::Logger;
use crate::process_control::{pause_process, resume_process, kill_process};
use super::temperature::get_max_temperature;
use super::cpu::{check_cpu_usage, is_cpu_overload_sustained};

use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::System;
use systemstat::{Platform, System as StatSystem};

pub fn monitor_ia(ia: IaConfig, logger: Logger) {
    println!("🚀 Lancement de l'IA '{}'", ia.name);
    
    // Paramètres de surveillance
    let settings = GuardianSettings::default();
    
    // Si besoin d'accéder à des chemins dans le workspace
    let mut child = Command::new("cargo")
        .arg("run")
        .arg("--manifest-path")
        // Assurons-nous que le chemin est relatif à la racine du workspace
        .arg(&ia.manifest_path) // Déjà relatif à la racine dans le fichier de config
        .args(&ia.args)
        .spawn()
        .expect("Impossible de lancer l'IA");

    let pid = child.id();
    println!("Processus IA '{}' lancé avec PID {}", ia.name, pid);

    let mut system = System::new_all();
    let stat_sys = StatSystem::new();

    loop {
        thread::sleep(Duration::from_millis(settings.check_interval_ms));
        system.refresh_all();

        // Vérification de la température
        let max_temp = get_max_temperature(&stat_sys, &ia.name);
            
        // Action en cas de température critique
        if max_temp > settings.temp_threshold && max_temp > 0.0 {
            let message = format!("⚠️ ALERTE! Température critique ({:.1}°C) ! Arrêt d'urgence de l'IA {}.", max_temp, ia.name);
            println!("{}", message);
            
            logger.log(&ia.name, 0.0, max_temp, "KILL", &message);
            kill_process(pid);
            
            println!("⏳ Attente de refroidissement... (30s)");
            thread::sleep(Duration::from_secs(30));
            break;
        }
        
        // Vérification CPU
        let (cpu_overload, process_cpu) = check_cpu_usage(&mut system, pid, settings.cpu_threshold);

        // Vérification supplémentaire - surcharge durable
        if !cpu_overload && is_cpu_overload_sustained(&mut system, pid, settings.cpu_threshold - 10.0, 3) {
            // Détection d'une surcharge moins importante mais persistante
            let message = format!("[{}] Surcharge CPU persistante détectée", ia.name);
            println!("{}", message);
            logger.log(&ia.name, process_cpu, max_temp, "PERSISTENT_LOAD", &message);
        }
        
        // Action en cas de surcharge CPU
        if cpu_overload {
            let message = format!("🔥 [{}] Surcharge CPU détectée! Refroidissement en cours...", ia.name);
            println!("{}", message);
            
            logger.log(&ia.name, process_cpu, max_temp, "PAUSE", &message);
            
            pause_process(pid);
            thread::sleep(Duration::from_millis(settings.cool_down_duration));
            
            // Double vérification température
            let cooling_temp = get_max_temperature(&stat_sys, &ia.name);
            
            if cooling_temp > settings.temp_threshold - 5.0 {
                let message = format!("🛑 [{}] Température encore trop haute ({:.1}°C). Extension du refroidissement.", ia.name, cooling_temp);
                println!("{}", message);
                logger.log(&ia.name, process_cpu, cooling_temp, "EXTENDED_PAUSE", &message);
                thread::sleep(Duration::from_secs(20));
            }
            
            let message = format!("[{}] Reprise de l'IA après pause", ia.name);
            println!("{}", message);
            logger.log(&ia.name, process_cpu, max_temp, "RESUME", &message);
            
            resume_process(pid);
            continue;
        }

        // Vérification si le processus est toujours en vie
        if let Some(status) = child.try_wait().unwrap() {
            let message = format!("Processus IA '{}' terminé avec status {:?}", ia.name, status);
            println!("{}", message);
            logger.log(&ia.name, 0.0, max_temp, "TERMINATED", &message);
            break;
        }
    }
}
