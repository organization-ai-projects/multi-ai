use serde::{Serialize, Deserialize};
use crate::graph::DependencyGraph;
use crate::watcher::VersionWatcher;
use axum::extract::State;
use axum::response::Json;
use axum::routing::{get, post};
use axum::{Router};
use hyper::Server;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex as StdMutex};
use tokio::sync::{broadcast, Mutex as TokioMutex};

#[derive(Debug, Serialize, Clone)]
pub struct VersionUpdate {
    pub version: String,
    pub changes: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize)]
pub struct TriggerRequest {
    pub project_path: String,
    pub force: bool,
}

#[derive(Clone)]
pub struct ApiState {
    pub graph: Arc<TokioMutex<DependencyGraph>>,
    pub watcher: Arc<StdMutex<VersionWatcher>>,
    pub updates_tx: broadcast::Sender<VersionUpdate>,
}

pub async fn start_api_server(state: ApiState) {
    let app = Router::new()
        .route("/status", get(status))
        .route("/bump", post(manual_bump))
        .route("/graph", get(get_graph))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("API server starting on {}", addr);
    
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn status(State(state): State<ApiState>) -> Json<serde_json::Value> {
    let watcher = state.watcher.lock().unwrap();
    Json(serde_json::json!({
        "status": "running",
        "dry_run": watcher.config.dry_run,
        "auto_commit": watcher.config.auto_commit
    }))
}

async fn manual_bump(State(state): State<ApiState>) -> Json<serde_json::Value> {
    // ... implémentation de bump manuel ...
    Json(serde_json::json!({"status": "triggered"}))
}

async fn get_graph(State(state): State<ApiState>) -> Json<serde_json::Value> {
    let graph = state.graph.lock().await;
    Json(serde_json::json!({
        "graph": graph.to_json()
    }))
}
