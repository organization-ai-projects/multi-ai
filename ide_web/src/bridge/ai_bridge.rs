use crate::bridge::errors::{BridgeError, explain_error, BridgeResult}; // Maintien des erreurs
use lazy_static::lazy_static;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;

lazy_static! {
    static ref MEMORY_MANAGER: Arc<Mutex<()>> = Arc::new(Mutex::new(())); // Placeholder pour la mémoire
}

/// Helper pour verrouiller la mémoire en toute sécurité
pub fn lock_memory() -> BridgeResult<std::sync::MutexGuard<'static, ()>> {
    MEMORY_MANAGER
        .lock()
        .map_err(|e| {
            let error = BridgeError::from(e);
            eprintln!("Erreur (code: {}): {}", error.code(), explain_error(&error));
            explain_error(&error)
        })
}

/// Fournit un accès unique à la mémoire chargée
pub fn get_memory() -> Arc<Mutex<()>> {
    MEMORY_MANAGER.clone()
}

/// Sauvegarde la mémoire actuelle
pub fn save_memory() -> BridgeResult<()> {
    let memory = lock_memory()?;
    memory
        .save_memory()
        .map_err(|e| {
            let error = BridgeError::Io(e);
            eprintln!("Erreur (code: {}): {}", error.code(), explain_error(&error));
            explain_error(&error)
        })
}

pub fn get_suggestion(file_path: &str) -> BridgeResult<Option<String>> {
    Ok(ai_assistant::agent::run_snapshot(file_path))
}

pub fn analyze_changes(files: &[String]) -> BridgeResult<Value> {
    let analysis = analyze_snapshot(files);
    Ok(json!({
        "impact": analysis.impact,
        "area": analysis.dominant_area,
        "files": analysis.files_changed
    }))
}

pub fn analyze_version_pattern(history: &Value) -> BridgeResult<String> {
    let memory = lock_memory()?;
    if memory.nodes.is_empty() {
        let error = BridgeError::IA("Mémoire IA vide".into());
        eprintln!("Erreur (code: {}): {}", error.code(), explain_error(&error));
        return Err(explain_error(&error));
    }
    Ok(analyze_version_pattern(history, &memory))
}

pub fn determine_best_strategy(history: &Value) -> BridgeResult<String> {
    let memory = lock_memory()?;
    if memory.nodes.is_empty() {
        let error = BridgeError::IA("Mémoire IA vide".into());
        eprintln!("Erreur (code: {}): {}", error.code(), explain_error(&error));
        return Err(explain_error(&error));
    }
    Ok(determine_best_strategy(history, &memory))
}

/// Envoie une commande à l'IA via WebSocket et retourne la réponse
pub async fn send_ai_command(command: &str) -> Result<String, String> {
    let (mut socket, _) = tokio_tungstenite::connect_async("ws://localhost:4001/ws").await.map_err(|e| e.to_string())?;
    socket.send(tokio_tungstenite::tungstenite::protocol::Message::Text(command.to_string()))
        .await
        .map_err(|e| e.to_string())?;

    if let Some(Ok(tokio_tungstenite::tungstenite::protocol::Message::Text(response))) = socket.next().await {
        Ok(response)
    } else {
        Err("Aucune réponse de l'IA".to_string())
    }
}

/// Analyse les changements dans les fichiers
pub async fn analyze_changes(files: &[String]) -> Result<String, String> {
    let command = format!("analyze_changes:{}", serde_json::to_string(files).unwrap());
    send_ai_command(&command).await
}

/// Analyse le pattern de version
pub async fn analyze_version_pattern(history: &serde_json::Value) -> Result<String, String> {
    let command = format!("analyze_version_pattern:{}", history.to_string());
    send_ai_command(&command).await
}

/// Détermine la meilleure stratégie de version
pub async fn determine_best_strategy(history: &serde_json::Value) -> Result<String, String> {
    let command = format!("determine_best_strategy:{}", history.to_string());
    send_ai_command(&command).await
}

/// Analyse combinée des fichiers et de l'historique
pub async fn analyze_combined(files: &[String], history: &serde_json::Value) -> Result<serde_json::Value, String> {
    let command = serde_json::json!({
        "files": files,
        "history": history,
    });
    let command = format!("analyze_combined:{}", command.to_string());
    let response = send_ai_command(&command).await?;
    serde_json::from_str(&response).map_err(|e| e.to_string())
}
