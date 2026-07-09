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

        if text == "suggest" {
            let suggestion = "Suggestion IA : bump version".to_string();
            if socket.send(Message::Text(suggestion)).await.is_err() {
                break;
            }
        } else if text.starts_with("analyze_combined:") {
            let payload: serde_json::Value = serde_json::from_str(&text["analyze_combined:".len()..]).unwrap();
            let files = payload["files"].as_array().unwrap();
            let history = &payload["history"];

            let changes = analyze_snapshot(files);
            let version_pattern = analyze_version_pattern(history);
            let strategy = determine_best_strategy(history);

            let response = serde_json::json!({
                "changes": changes,
                "version_pattern": version_pattern,
                "strategy": strategy,
            });

            if socket.send(Message::Text(response.to_string())).await.is_err() {
                break;
            }
        }
    }
}
