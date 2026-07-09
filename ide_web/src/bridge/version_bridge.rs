use serde_json::Value;
use crate::bridge::errors::{BridgeError, explain_error, BridgeResult};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

pub fn revert(id: &str) -> BridgeResult<()> {
    delete_snapshot(id).map_err(|e| {
        let error = BridgeError::Io(e);
        eprintln!("Erreur (code: {}): {}", error.code(), explain_error(&error));
        explain_error(&error)
    })
}

/// Envoie une commande au gestionnaire de versions via WebSocket et retourne la réponse
pub async fn send_version_command(command: &str) -> Result<String, String> {
    let (mut socket, _) = tokio_tungstenite::connect_async("ws://localhost:4002/ws").await.map_err(|e| e.to_string())?;
    socket.send(tokio_tungstenite::tungstenite::protocol::Message::Text(command.to_string()))
        .await
        .map_err(|e| e.to_string())?;

    if let Some(Ok(tokio_tungstenite::tungstenite::protocol::Message::Text(response))) = socket.next().await {
        Ok(response)
    } else {
        Err("Aucune réponse du gestionnaire de versions".to_string())
    }
}

pub async fn get_version_graph() -> Result<String, String> {
    send_version_command("get_version_graph").await
}

pub async fn bump_version() -> Result<String, String> {
    send_version_command("bump_version").await
}

pub async fn revert_to_snapshot(snapshot_id: &str) -> Result<String, String> {
    let command = format!("revert_to:{}", snapshot_id);
    send_version_command(&command).await
}

pub fn get_latest_snapshot_id() -> BridgeResult<String> {
    get_latest_snapshot_id().ok_or_else(|| {
        let error = BridgeError::IA("Aucun snapshot trouvé.".to_string());
        eprintln!("Erreur (code: {}): {}", error.code(), explain_error(&error));
        explain_error(&error)
    })
}
