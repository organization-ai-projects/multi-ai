use crate::watcher::start_watching;
use crate::graph::{print_graph_log, print_latest_snapshot, revert_to_snapshot};

pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("watch") => {
            let default_path = ".".to_string();
            let path = args.get(2).unwrap_or(&default_path); // Utilisation d'une variable pour prolonger la durée de vie
            start_watching(path);
        }
        Some("graph") => match args.get(2).map(String::as_str) {
            Some("log") => print_graph_log(),
            Some("latest") => print_latest_snapshot(),
            Some("revert") => {
                if let Some(id) = args.get(3) {
                    revert_to_snapshot(id);
                } else {
                    eprintln!("❌ ID manquant pour la commande revert.");
                }
            }
            _ => eprintln!("❌ Commande inconnue pour 'graph'."),
        },
        _ => eprintln!("❌ Commande inconnue."),
    }
}
