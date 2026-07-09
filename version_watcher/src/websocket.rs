use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::StreamExt;

pub async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        println!("Message reçu : {}", text);

        if text == "bump_version" {
            let response = "Version bumped successfully!".to_string();
            if socket.send(Message::Text(response)).await.is_err() {
                break;
            }
        }
    }
}
