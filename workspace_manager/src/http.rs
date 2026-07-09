//! Serveur HTTP et API REST pour le Workspace Manager
//!
//! Ce module fournit :
//! - Un serveur HTTP sur le port 4000
//! - Des endpoints REST pour gérer les services (démarrage, arrêt, liste)
//! - Un endpoint WebSocket pour les communications en temps réel
//! - Un système pour servir les fichiers statiques (UI web)
//! - Des fonctions pour communiquer avec d'autres services via WebSocket
//!
//! Usage:
//! - `GET /` : Interface utilisateur principale
//! - `GET /api/services` : Liste tous les services et leur état
//! - `POST /api/service/start/:name` : Démarre un service spécifique
//! - `POST /api/service/stop/:name` : Arrête un service spécifique
//! - `POST /api/command/:service/:command` : Envoie une commande à un service spécifique
//! - WebSocket sur `/ws` : Communication bidirectionnelle en temps réel
//! - Fichiers statiques dans `/static/*`

use crate::services::ServiceManager;
use crate::websocket::websocket_handler;
use axum::{
    extract::Path,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn start_http_server() {
    let (tx, _rx) = broadcast::channel(100);
    let service_manager = Arc::new(Mutex::new(ServiceManager::new()));

    // Création du Router pour l'API
    let api_router = Router::new()
        .route(
            "/services",
            get({
                let service_manager = service_manager.clone();
                move || list_services(service_manager.clone())
            }),
        )
        .route(
            "/service/start/:name",
            post({
                let service_manager = service_manager.clone();
                move |Path(name): Path<String>| start_service(service_manager.clone(), name)
            }),
        )
        .route(
            "/service/stop/:name",
            post({
                let service_manager = service_manager.clone();
                move |Path(name): Path<String>| stop_service(service_manager.clone(), name)
            }),
        )
        // Route pour envoyer des commandes à un service spécifique
        .route(
            "/command/:service/:command",
            post(|Path((service, command)): Path<(String, String)>| send_command(service, command)),
        );

    // Router principal
    let app = Router::new()
        .route("/ws", get(move |ws| websocket_handler(ws, tx.clone())))
        .route("/", get(serve_ui))
        .nest("/api", api_router)
        // Route pour servir les fichiers statiques
        .route("/static/*path", get(serve_static_file));

    let addr: SocketAddr = "0.0.0.0:4000".parse().unwrap();
    println!("Serveur démarré sur http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// Sert la page HTML principale
async fn serve_ui() -> Html<String> {
    // Utiliser uniquement le chemin spécifié
    let path = "workspace_manager/public/index.html";

    match std::fs::read_to_string(path) {
        Ok(content) => {
            println!("Interface chargée depuis : {}", path);
            Html(content)
        }
        Err(e) => {
            eprintln!(
                "ERREUR CRITIQUE : Impossible de trouver le fichier {} : {}",
                path, e
            );
            eprintln!("Le serveur ne peut pas fonctionner sans ce fichier. Assurez-vous qu'il existe au bon emplacement.");
            std::process::exit(1); // Arrêter le programme avec un code d'erreur
        }
    }
}

// Liste les services et leur état
async fn list_services(service_manager: Arc<Mutex<ServiceManager>>) -> Json<serde_json::Value> {
    let manager = service_manager.lock().unwrap();
    let services: Vec<_> = manager.list_services();
    Json(json!(services))
}

// Démarre un service
async fn start_service(
    service_manager: Arc<Mutex<ServiceManager>>,
    name: String,
) -> Json<serde_json::Value> {
    let mut manager = service_manager.lock().unwrap();
    let success = match name.as_str() {
        "ide_web" => manager.start_service(
            "ide_web",
            "cargo",
            &["run", "--manifest-path", "../ide_web/Cargo.toml"],
        ),
        "ai_assistant" => manager.start_service(
            "ai_assistant",
            "cargo",
            &["run", "--manifest-path", "../ai_assistant/Cargo.toml"],
        ),
        "version_watcher" => manager.start_service(
            "version_watcher",
            "cargo",
            &["run", "--manifest-path", "../version_watcher/Cargo.toml"],
        ),
        "semver_planner" => manager.start_service(
            "semver_planner",
            "cargo",
            &["run", "--manifest-path", "../semver_planner/Cargo.toml"],
        ),
        _ => {
            eprintln!("Service inconnu : {}", name);
            false
        }
    };
    Json(json!({ "success": success }))
}

// Arrête un service
async fn stop_service(
    service_manager: Arc<Mutex<ServiceManager>>,
    name: String,
) -> Json<serde_json::Value> {
    let mut manager = service_manager.lock().unwrap();
    let success = manager.stop_service(&name);
    Json(json!({ "success": success }))
}

// Handler pour servir les fichiers statiques
async fn serve_static_file(Path(path): Path<String>) -> impl axum::response::IntoResponse {
    let path = format!("public/{}", path);
    match tokio::fs::read(&path).await {
        Ok(data) => {
            let mime_type = match path.split('.').last() {
                Some("html") => "text/html",
                Some("css") => "text/css",
                Some("js") => "application/javascript",
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };
            ([(axum::http::header::CONTENT_TYPE, mime_type)], data)
        }
        Err(_) => (
            [(axum::http::header::CONTENT_TYPE, "text/plain")],
            "File not found".into(),
        ),
    }
}

// Fonction générique pour envoyer une commande via WebSocket
async fn send_websocket_command(url: &str, command: &str) -> Result<String, String> {
    let (mut socket, _) = connect_async(url).await.map_err(|e| e.to_string())?;
    socket
        .send(Message::Text(command.to_string()))
        .await
        .map_err(|e| e.to_string())?;

    if let Some(Ok(Message::Text(response))) = socket.next().await {
        Ok(response)
    } else {
        Err("Aucune réponse reçue".to_string())
    }
}

// Fonction pour envoyer une commande à un service spécifique
async fn send_command(service: String, command: String) -> Json<serde_json::Value> {
    let url = match service.as_str() {
        "ai_assistant" => "ws://localhost:4001/ws",
        "version_watcher" => "ws://localhost:4002/ws",
        "semver_planner" => "ws://localhost:4003/ws",
        "ide_web" => "ws://localhost:3000/ws",
        _ => return Json(json!({ "success": false, "error": "Service inconnu" })),
    };

    match send_websocket_command(url, &command).await {
        Ok(response) => Json(json!({ "success": true, "response": response })),
        Err(error) => Json(json!({ "success": false, "error": error })),
    }
}
