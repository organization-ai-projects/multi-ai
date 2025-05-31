mod graph_memory;
mod cli_tools;

use chrono;
use std::collections::HashMap;
use graph_memory::{Node, Edge, Orchestrator};
use cli_tools::{run_automate_projects, list_cli_tools, run_cli_tool};  // Nécessaire pour les options 5 et 6

fn main() {
    let project_base = "graph_memory_ai";
    let memory_dir = format!("{}/memory", project_base);
    let ron_path = format!("{}/memory.ron", memory_dir);
    let bin_path = format!("{}/memory.bin", memory_dir);
    
    std::fs::create_dir_all(&memory_dir).unwrap();
    
    let mut orchestrator = Orchestrator::load(&ron_path).unwrap_or_else(|_| Orchestrator::new());

    // On a besoin des outils CLI pour les options 5 et 6
    loop {
        println!("\n[graph_memory_ai] Que dois-je faire ?");
        println!("1. Lancer automate_projects (dry-run)");
        println!("2. Lancer automate_projects (apply)");
        println!("3. Afficher la mémoire");
        println!("4. Ajouter un noeud manuel");
        println!("5. Lister les CLI disponibles");
        println!("6. Exécuter un CLI du dossier cli_tools/");
        println!("7. Quitter");
        print!("Votre choix : ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut choix = String::new();
        std::io::stdin().read_line(&mut choix).unwrap();
        match choix.trim() {
            "1" => {
                match run_automate_projects(&["--dry-run", "--verbose"]) {
                    Ok(output) => {
                        println!("Sortie automate_projects :\n{}", output);
                        let run_id = format!("cli_run_{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
                        orchestrator.process_cli_output(&output, &run_id);
                        orchestrator.save(&ron_path).unwrap();
                        orchestrator.save_bin(&bin_path).unwrap();
                        println!("Mémoire graphique structurée et mise à jour !");
                    }
                    Err(e) => eprintln!("Erreur automate_projects : {}", e),
                }
            }
            "2" => {
                match run_automate_projects(&["--apply"]) {
                    Ok(output) => {
                        println!("Sortie automate_projects :\n{}", output);
                        let run_id = format!("cli_apply_{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
                        orchestrator.process_cli_output(&output, &run_id);
                        orchestrator.save(&ron_path).unwrap();
                        orchestrator.save_bin(&bin_path).unwrap();
                        println!("Mémoire graphique structurée et mise à jour !");
                    }
                    Err(e) => eprintln!("Erreur automate_projects : {}", e),
                }
            }
            "3" => {
                println!("--- MÉMOIRE GRAPHIQUE ---");
                for node in orchestrator.nodes() {
                    println!("Noeud {} [{}] : {:?}", node.id, node.label, node.properties);
                }
                for edge in orchestrator.edges() {
                    println!("Lien {} --({})-> {}", edge.from, edge.label, edge.to);
                }
            }
            "4" => {
                println!("1. Ajouter un noeud");
                println!("2. Ajouter un lien");
                print!("Votre choix : ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                
                let mut subchoix = String::new();
                std::io::stdin().read_line(&mut subchoix).unwrap();
                
                match subchoix.trim() {
                    "1" => {
                        // Existant: ajout de noeud
                        let mut id = String::new();
                        let mut label = String::new();
                        println!("ID du noeud : ");
                        std::io::stdin().read_line(&mut id).unwrap();
                        println!("Label du noeud : ");
                        std::io::stdin().read_line(&mut label).unwrap();
                        orchestrator.add_or_update_node(Node {
                            id: id.trim().to_string(),
                            label: label.trim().to_string(),
                            properties: HashMap::new(),
                        });
                    }
                    "2" => {
                        // Nouveau: ajout de lien
                        let mut from = String::new();
                        let mut to = String::new();
                        let mut label = String::new();
                        
                        println!("ID source : ");
                        std::io::stdin().read_line(&mut from).unwrap();
                        println!("ID destination : ");
                        std::io::stdin().read_line(&mut to).unwrap();
                        println!("Label du lien : ");
                        std::io::stdin().read_line(&mut label).unwrap();
                        
                        orchestrator.add_edge(Edge {
                            from: from.trim().to_string(),
                            to: to.trim().to_string(),
                            label: label.trim().to_string(),
                        });
                    }
                    _ => println!("Choix invalide")
                }
                
                orchestrator.save(&ron_path).unwrap();
                orchestrator.save_bin(&bin_path).unwrap();
                println!("Graphe mis à jour !");
            }
            "5" => {
                println!("--- CLI disponibles dans cli_tools/ ---");
                for tool in list_cli_tools() {  // Utilisation directe de list_cli_tools
                    println!("  - {}", tool);
                }
            }
            "6" => {
                let tools = list_cli_tools();  // Utilisation directe de list_cli_tools
                if tools.is_empty() {
                    println!("Aucun outil CLI trouvé dans cli_tools/");
                    continue;
                }
                println!("Outils disponibles :");
                for (i, tool) in tools.iter().enumerate() {
                    println!("{}: {}", i + 1, tool);
                }
                print!("Numéro de l'outil à exécuter : ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut idx = String::new();
                std::io::stdin().read_line(&mut idx).unwrap();
                let idx: usize = idx.trim().parse().unwrap_or(0);
                if idx == 0 || idx > tools.len() {
                    println!("Numéro invalide.");
                    continue;
                }
                print!("Arguments (séparés par des espaces) : ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut args = String::new();
                std::io::stdin().read_line(&mut args).unwrap();
                let args: Vec<&str> = args.trim().split_whitespace().collect();
                match run_cli_tool(&tools[idx - 1], &args) {  // Utilisation directe de run_cli_tool
                    Ok(output) => {
                        println!("Sortie stdout :\n{}", String::from_utf8_lossy(&output.stdout));
                        println!("Sortie stderr :\n{}", String::from_utf8_lossy(&output.stderr));
                    }
                    Err(e) => println!("Erreur : {}", e),
                }
            }
            "7" => {
                println!("Arrêt de l'agent.");
                break;
            }
            _ => println!("Choix inconnu."),
        }
    }
}
