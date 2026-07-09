use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{self, AsyncBufReadExt};
use log::error;

use crate::monitoring::monitor::ResourceMonitor;
use crate::models::security::SecurityLevel;

pub struct CliController {
    monitor: Arc<Mutex<ResourceMonitor>>,
}

impl CliController {
    pub fn new(monitor: Arc<Mutex<ResourceMonitor>>) -> Self {
        Self { monitor }
    }
    
    pub async fn start(&self) {
        let stdin = io::stdin();
        let handle = stdin.lock();
        let mut lines = handle.lines();
        
        println!("Contrôleur CLI démarré. Tapez 'help' pour les commandes.");
        
        while let Some(line) = lines.next_line().await.unwrap() {
            self.handle_command(line).await;
        }
    }
    
    async fn handle_command(&self, line: String) {
        let args: Vec<&str> = line.trim().split_whitespace().collect();
        
        if args.is_empty() {
            return;
        }
        
        match args[0] {
            "help" => {
                println!("Commandes disponibles:");
                println!("  help          - Affiche cette aide");
                println!("  status        - Affiche l'état actuel du moniteur");
                println!("  set_priority  - Définit la priorité d'une IA (usage: set_priority <id> <priorité>)");
                println!("  pause         - Met une IA en pause (usage: pause <id>)");
                println!("  resume        - Reprend une IA (usage: resume <id>)");
                println!("  set_security  - Définit le niveau de sécurité (usage: set_security <niveau>)");
                println!("  exit          - Quitte le contrôleur");
            },
            "status" => {
                self.print_status().await;
            },
            "set_priority" => {
                if args.len() != 3 {
                    println!("Usage: set_priority <id> <priorité>");
                    return;
                }
                
                let id = args[1];
                let priority = args[2].parse::<u8>();
                
                match priority {
                    Ok(p) => {
                        if let Err(e) = self.monitor.lock().await.set_ai_priority(id, p).await {
                            error!("Erreur lors de la définition de la priorité: {}", e);
                        }
                    },
                    Err(_) => {
                        println!("La priorité doit être un nombre.");
                    }
                }
            },
            "pause" => {
                if args.len() != 2 {
                    println!("Usage: pause <id>");
                    return;
                }
                
                let id = args[1];
                
                if let Err(e) = self.monitor.lock().await.pause_ai(id).await {
                    error!("Erreur lors de la mise en pause de l'IA: {}", e);
                }
            },
            "resume" => {
                if args.len() != 2 {
                    println!("Usage: resume <id>");
                    return;
                }
                
                let id = args[1];
                
                if let Err(e) = self.monitor.lock().await.resume_ai(id).await {
                    error!("Erreur lors de la reprise de l'IA: {}", e);
                }
            },
            "set_security" => {
                if args.len() != 2 {
                    println!("Usage: set_security <niveau>");
                    return;
                }
                
                let level = match args[1] {
                    "standard" => SecurityLevel::Standard,
                    "high" => SecurityLevel::High,
                    "critical" => SecurityLevel::Critical,
                    _ => {
                        println!("Niveau de sécurité inconnu. Utilisez 'standard', 'high' ou 'critical'.");
                        return;
                    }
                };
                
                self.monitor.lock().await.set_security_level(level);
            },
            "exit" => {
                println!("Fermeture du contrôleur.");
                std::process::exit(0);
            },
            _ => {
                println!("Commande inconnue: {}", args[0]);
            }
        }
    }
    
    async fn print_status(&self) {
        let monitor = self.monitor.lock().await;
        
        println!("État actuel du moniteur:");
        println!("  Instances d'IA surveillées: {}", monitor.get_ai_instances().len());
        println!("  Processus actifs: {}", monitor.get_process_map().len());
        
        for ai in monitor.get_ai_instances() {
            println!("  - IA ID: {}, État: {:?}, Priorité: {}", 
                     ai.id, ai.state, ai.priority);
        }
    }
}