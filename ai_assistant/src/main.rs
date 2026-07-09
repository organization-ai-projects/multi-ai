use ai_assistant::cli::parse_and_execute;
use ai_assistant::graph_memory::AiGraph;
use ai_assistant::version::VersionSnapshot;
use ai_assistant::agent::run_agent;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::io;
use axum::{Router, routing::get};
use ai_assistant::websocket::websocket_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(websocket_handler));

    println!("🚀 AI Assistant WebSocket lancé sur ws://localhost:4001/ws");
    axum::Server::bind(&"0.0.0.0:4001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn main() {
    let stop_signal = Arc::new(AtomicBool::new(false));
    let stop_signal_clone = Arc::clone(&stop_signal);

    // Gestion du signal d'arrêt via un thread séparé
    thread::spawn(move || {
        let mut input = String::new();
        println!("Tapez 'stop' pour arrêter l'IA.");
        while io::stdin().read_line(&mut input).is_ok() {
            if input.trim() == "stop" {
                stop_signal_clone.store(true, Ordering::Relaxed);
                break;
            }
            input.clear();
        }
    });

    // Initialisation de l'IA
    let mut graph = AiGraph::new();
    let snapshot = VersionSnapshot {
        id: "v1.0.0".to_string(),
        files_changed: vec!["lib.rs".to_string()],
    };

    // Lancer l'agent IA
    run_agent(&snapshot, &mut graph, stop_signal);

    // Sauvegarde de la mémoire avant l'arrêt
    if let Err(e) = graph.save_to_file("graph_memory.ron") {
        eprintln!("Erreur lors de la sauvegarde en .ron : {}", e);
    }

    if let Err(e) = graph.save_to_bin("graph_memory.bin") {
        eprintln!("Erreur lors de la sauvegarde en .bin : {}", e);
    }

    println!("Mémoire sauvegardée dans 'graph_memory.ron' et 'graph_memory.bin'.");

    // Lancer l'interface CLI
    parse_and_execute();
}
