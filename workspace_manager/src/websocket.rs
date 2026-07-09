use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::broadcast;
use futures_util::{SinkExt, StreamExt};

pub async fn websocket_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<String>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, tx: broadcast::Sender<String>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = tx.subscribe();

    tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            println!("Message reçu : {}", text);
            let _ = tx.send(text.to_string());
        }
    });

    while let Ok(message) = rx.recv().await {
        if sender.feed(Message::Text(message.into())).await.is_err() { // Utilisation de feed au lieu de send
            break;
        }
        if sender.flush().await.is_err() { // Ajout de flush après feed
            break;
        }
    }
}
