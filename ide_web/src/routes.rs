use axum::{
    Router, Json,
    routing::{get, post},
    extract::Path
};
use serde::{Deserialize, Serialize};
use crate::{controller, filesystem};
use crate::filesystem::list_files;
use crate::websocket::{websocket_handler, send_contextual_suggestion};
use tokio::sync::broadcast;
use crate::bridge::smart_bridge::{smart_version_bump, suggest_version_strategy};
use crate::plugins::{auto_comment_function, auto_format_code, summarize_file};
use ai_assistant::agent::run_snapshot;
use reqwest::Client;

pub fn routes(tx: broadcast::Sender<String>) -> Router {
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    
    Router::new()
        .route("/file/get/:path", get(get_file))
        .route("/file/save", post(save_file))
        .route("/suggest", get(suggest))
        .route("/bump", post(bump))
        .route("/revert/:id", post(revert))
        .route("/files", get(list_files_handler))
        .route("/ws/suggestions", get(move |ws| websocket_handler(ws, tx1.clone())))
        .route("/ws/trigger", post(move || controller::trigger_suggestion(tx2.clone())))
        .route("/graph/ui", get(graph_ui))
        .route("/revert/visual/:id", post(revert_visual_handler))
        .route("/ws/contextual-suggestion/:file", post(move |Path(file): Path<String>| {
            let tx = tx3.clone();
            async move {
                send_contextual_suggestion(tx, &file).await;
                Json(Status { success: true })
            }
        }))
        .route("/ai/prompt", post(ai_prompt))
        .route("/plugin/comment", post(apply_auto_comment))
        .route("/plugin/format", post(apply_auto_format))
        .route("/plugin/summarize", post(apply_summarize))
        .route("/plugin/apply", post(apply_plugin))
        .route("/languages", get(get_supported_languages))
        .route("/ws", get(move |ws| websocket_handler(ws, tx1.clone())))
        .route("/smart/bump", post(smart_bump_handler))
        .route("/smart/strategy", post(smart_strategy_handler))
}

async fn get_file(Path(path): Path<String>) -> Json<FileContent> {
    let content = filesystem::read_file(&path).unwrap_or("".to_string());
    Json(FileContent { path, content })
}

#[derive(Deserialize)]
struct SaveRequest {
    path: String,
    content: String,
}

async fn save_file(Json(req): Json<SaveRequest>) -> Json<Status> {
    let success = filesystem::save_file_with_snapshot(&req.path, &req.content).is_ok();
    Json(Status { success })
}

async fn bump() -> Json<Status> {
    let success = smart_version_bump().is_ok();
    Json(Status { success })
}

async fn revert(Path(id): Path<String>) -> Json<Status> {
    let success = controller::revert_to(&id).is_ok();
    Json(Status { success })
}

async fn list_files_handler() -> Json<serde_json::Value> {
    let files = list_files("src").unwrap_or_default();
    Json(serde_json::json!({ "files": files }))
}

async fn graph_ui() -> Json<serde_json::Value> {
    Json(get_graph_ui().unwrap_or_default())
}

async fn revert_visual_handler(Path(id): Path<String>) -> Json<Status> {
    let success = revert_visual(&id) == "↩️ Revert visuel effectué";
    Json(Status { success })
}

async fn suggest() -> Json<serde_json::Value> {
    if let Some(strategy) = suggest_version_strategy() {
        Json(strategy)
    } else {
        Json(serde_json::json!({ "error": "Impossible de générer une stratégie." }))
    }
}

async fn apply_plugin(Json(req): Json<SaveRequest>) -> Json<FileContent> {
    let updated_content = auto_comment_function(&req.content).unwrap_or_else(|e| e);
    Json(FileContent {
        path: req.path,
        content: updated_content,
    })
}

async fn ai_prompt(Json(req): Json<PromptRequest>) -> Json<PromptResponse> {
    let response = run_snapshot(&req.prompt).unwrap_or("L'IA n'a pas de réponse.".to_string());
    Json(PromptResponse { response })
}

async fn apply_auto_comment(Json(req): Json<SaveRequest>) -> Json<FileContent> {
    match auto_comment_function(&req.content) {
        Ok(updated_content) => Json(FileContent {
            path: req.path,
            content: updated_content,
        }),
        Err(err) => Json(FileContent {
            path: req.path,
            content: format!("Erreur : {}", err),
        }),
    }
}

async fn apply_auto_format(Json(req): Json<SaveRequest>) -> Json<FileContent> {
    match auto_format_code(&req.content) {
        Ok(updated_content) => Json(FileContent {
            path: req.path,
            content: updated_content,
        }),
        Err(err) => Json(FileContent {
            path: req.path,
            content: format!("Erreur : {}", err),
        }),
    }
}

async fn apply_summarize(Json(req): Json<SaveRequest>) -> Json<FileContent> {
    match summarize_file(&req.content) {
        Ok(summary) => Json(FileContent {
            path: req.path,
            content: summary,
        }),
        Err(err) => Json(FileContent {
            path: req.path,
            content: format!("Erreur : {}", err),
        }),
    }
}

async fn get_supported_languages() -> Json<Vec<String>> {
    Json(vec![
        "rust".to_string(),
        "javascript".to_string(),
        "python".to_string(),
        "html".to_string(),
        "css".to_string(),
    ])
}

/// Handler pour le bump de version intelligent
async fn smart_bump_handler() -> Json<serde_json::Value> {
    match smart_version_bump().await {
        Ok(result) => Json(serde_json::json!({ "success": true, "result": result })),
        Err(err) => Json(serde_json::json!({ "success": false, "error": err })),
    }
}

/// Handler pour la stratégie de version intelligente
async fn smart_strategy_handler() -> Json<serde_json::Value> {
    match suggest_version_strategy().await {
        Ok(strategy) => Json(serde_json::json!({ "success": true, "strategy": strategy })),
        Err(err) => Json(serde_json::json!({ "success": false, "error": err })),
    }
}
