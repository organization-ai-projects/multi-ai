use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{info, warn, error, debug};
use tokio::time;
use sysinfo::{System, SystemExt};

use crate::models::resources::ResourceThresholds;
use crate::monitoring::monitor::ResourceMonitor;
use crate::monitoring::security_manager::SecurityManager;
use crate::monitoring::process_tracker::ProcessTracker;
use crate::error::ResourceMonitorError;
use crate::models::security::SecurityLevel;

/// Démarre la boucle principale de surveillance
pub async fn start_monitoring_loop(
    system: Arc<Mutex<System>>,
    thresholds: ResourceThresholds,
    is_running: Arc<Mutex<bool>>,
    monitor: ResourceMonitor,
    security_manager: SecurityManager,
    process_tracker: ProcessTracker,
) -> Result<(), ResourceMonitorError> {
    {
        let mut running = is_running.lock().unwrap();
        *running = true;
    }
    
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(thresholds.sampling_interval_ms));
        let mut last_action = Instant::now();
        let cooldown = Duration::from_millis(thresholds.cooldown_period_ms);
        
        // Intervalle spécifique pour la vérification de sécurité (moins fréquente)
        let mut security_check_interval = time::interval(Duration::from_secs(5));
        
        loop {
            interval.tick().await;
            
            {
                if !*is_running.lock().unwrap() {
                    info!("Arrêt de la boucle de surveillance");
                    break;
                }
            }
            
            // Rafraîchir les informations système
            {
                let mut sys = system.lock().unwrap();
                sys.refresh_all();
            }
            
            // Vérification principale des ressources
            check_system_resources(&monitor, &thresholds, &mut last_action, cooldown).await;
            
            // Vérifications de sécurité (moins fréquentes)
            if security_check_interval.tick().await.is_zero() {
                check_security_aspects(&system, &monitor, &security_manager, &mut process_tracker).await;
            }
        }
    });
    
    Ok(())
}

/// Vérifie les ressources système et prend des mesures si nécessaire
async fn check_system_resources(
    monitor: &ResourceMonitor,
    thresholds: &ResourceThresholds,
    last_action: &mut Instant,
    cooldown: Duration,
) {
    match monitor.get_system_stats() {
        Ok((cpu_usage, mem_usage)) => {
            debug!("CPU: {:.1}%, Mémoire: {:.1}%", cpu_usage, mem_usage);
            
            let resources_exceeded = cpu_usage > thresholds.max_cpu_usage || 
                                    mem_usage > thresholds.max_memory_usage;
            
            if resources_exceeded && last_action.elapsed() > cooldown {
                // Alerter sur le dépassement de ressources
                if cpu_usage > thresholds.max_cpu_usage {
                    warn!("Seuil CPU dépassé: {:.1}%, maximum: {:.1}%", cpu_usage, thresholds.max_cpu_usage);
                }
                
                if mem_usage > thresholds.max_memory_usage {
                    warn!("Seuil mémoire dépassé: {:.1}%, maximum: {:.1}%", mem_usage, thresholds.max_memory_usage);
                }
                
                // Déléguer la décision de mise en pause à la fonction spécialisée
                if handle_resource_exceeded(monitor).await {
                    *last_action = Instant::now();
                }
            }
            // Gestion de la reprise des IA en pause si les ressources sont libres
            else if !resources_exceeded && last_action.elapsed() > cooldown {
                if handle_resources_available(monitor).await {
                    *last_action = Instant::now();
                }
            }
        },
        Err(e) => error!("Erreur lors de la récupération des statistiques système: {}", e),
    }
}

/// Gère la situation où les ressources sont dépassées
async fn handle_resource_exceeded(monitor: &ResourceMonitor) -> bool {
    // Logique pour sélectionner et mettre en pause une IA
    let ai_instances = monitor.get_ai_instances();
    
    // Trouver les IA en cours d'exécution
    let running_instances: Vec<_> = ai_instances.iter()
        .filter(|ai| ai.is_running())
        .collect();
    
    if running_instances.is_empty() {
        return false;
    }
    
    // Trier par priorité (la plus basse d'abord)
    let mut sorted_instances = running_instances.clone();
    sorted_instances.sort_by_key(|ai| ai.priority);
    
    // Mettre en pause l'IA avec la priorité la plus basse
    if let Some(ai_to_pause) = sorted_instances.first() {
        warn!("Mise en pause automatique de l'IA {} (priorité {})", 
              ai_to_pause.id, ai_to_pause.priority);
        
        if let Err(e) = monitor.pause_ai(&ai_to_pause.id) {
            error!("Échec de la mise en pause de l'IA {}: {}", ai_to_pause.id, e);
            return false;
        }
        return true;
    }
    
    false
}

/// Gère la situation où les ressources sont disponibles
async fn handle_resources_available(monitor: &ResourceMonitor) -> bool {
    // Logique pour reprendre une IA en pause
    let ai_instances = monitor.get_ai_instances();
    
    // Trouver les IA en pause
    let paused_instances: Vec<_> = ai_instances.iter()
        .filter(|ai| ai.is_paused())
        .collect();
    
    if paused_instances.is_empty() {
        return false;
    }
    
    // Trier par priorité (la plus haute d'abord)
    let mut sorted_instances = paused_instances.clone();
    sorted_instances.sort_by_key(|ai| std::cmp::Reverse(ai.priority));
    
    // Reprendre l'IA avec la priorité la plus haute
    if let Some(ai_to_resume) = sorted_instances.first() {
        info!("Reprise automatique de l'IA {} (priorité {})", 
              ai_to_resume.id, ai_to_resume.priority);
        
        if let Err(e) = monitor.resume_ai(&ai_to_resume.id) {
            error!("Échec de la reprise de l'IA {}: {}", ai_to_resume.id, e);
            return false;
        }
        return true;
    }
    
    false
}

/// Vérifie les aspects de sécurité du système
async fn check_security_aspects(
    system: &Arc<Mutex<System>>,
    monitor: &ResourceMonitor,
    security_manager: &SecurityManager,
    process_tracker: &mut ProcessTracker,
) {
    // Vérifier les nouveaux processus enfants
    let sys = system.lock().unwrap();
    let new_children = process_tracker.check_for_new_child_processes(&sys);
    drop(sys);
    
    // Gérer les nouveaux processus enfants
    for (child_pid, parent_pid) in new_children {
        // Vérifier si le parent a dépassé sa limite de processus
        let child_count = process_tracker.count_children_for_parent(parent_pid);
        let max_allowed = process_tracker.max_processes_per_ai;
        
        if child_count >= max_allowed {
            error!("Limite de processus dépassée pour l'IA parent {}. Terminaison du processus enfant {}.", 
                   parent_pid, child_pid);
                   
            // Le niveau de sécurité détermine si on termine le processus
            if security_manager.get_security_level() == SecurityLevel::Critical {
                if let Err(e) = monitor.controller.terminate_process(child_pid) {
                    error!("Impossible de terminer le processus {}: {}", child_pid, e);
                }
            }
        } else {
            // Enregistrer le processus enfant
            process_tracker.register_child_process(child_pid, parent_pid);
        }
    }
    
    // Vérifier les modifications de fichiers non autorisées
    if let Ok(changes) = security_manager.check_for_unauthorized_changes() {
        if !changes.is_empty() {
            warn!("Modifications non autorisées détectées: {:?}", changes);
            
            // En fonction du niveau de sécurité, prendre des mesures
            match security_manager.get_security_level() {
                SecurityLevel::Standard => {
                    // Juste logguer l'événement
                    warn!("Modification non autorisée détectée (niveau standard): {:?}", changes);
                },
                SecurityLevel::High | SecurityLevel::Critical => {
                    // Mettre en pause toutes les IA
                    error!("ALERTE CRITIQUE: Modifications non autorisées détectées. Mise en pause d'urgence.");
                    if let Err(e) = monitor.emergency_pause_all() {
                        error!("Erreur lors de la mise en pause d'urgence: {}", e);
                    }
                }
            }
        }
    }
    
    // Vérifier les accès aux chemins interdits
    if let Ok(true) = security_manager.check_forbidden_paths() {
        error!("ALERTE MAJEURE: Tentative d'accès à un chemin interdit!");
        
        // Mesures d'urgence
        if let Err(e) = monitor.emergency_pause_all() {
            error!("Erreur lors de la mise en pause d'urgence suite à un accès interdit: {}", e);
        }
    }
}
