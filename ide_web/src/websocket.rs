use ai_assistant::agent::run_snapshot;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::broadcast;
use futures_util::StreamExt;

pub async fn websocket_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<String>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(mut socket: WebSocket, tx: broadcast::Sender<String>) {
    let mut rx = tx.subscribe();

    // Recevoir les messages du client
    tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = socket.next().await {
            println!("Message reçu : {}", text);
            // Diffuser le message à tous les abonnés
            let _ = tx.send(text);
        }
    });

    // Envoyer les messages aux clients
    while let Ok(message) = rx.recv().await {
        if socket.send(Message::Text(message)).await.is_err() {
            break;
        }
    }
}

pub async fn send_suggestion(tx: broadcast::Sender<String>, suggestion: String) {
    let _ = tx.send(suggestion);
}

pub async fn send_contextual_suggestion(tx: broadcast::Sender<String>, file_path: &str) {
    if let Some(suggestion) = run_snapshot(file_path) {
        let _ = tx.send(suggestion);
    }
}
