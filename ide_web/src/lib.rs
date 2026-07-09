pub mod routes;
pub mod filesystem;
pub mod controller;
pub mod websocket;
pub mod bridge; // Ajout du module `bridge`
pub mod plugins; // Ajout du module `plugins`

use axum::Router;
use routes::routes;
use tokio::sync::broadcast;

pub fn create_router(tx: broadcast::Sender<String>) -> Router {
    routes(tx)
}
