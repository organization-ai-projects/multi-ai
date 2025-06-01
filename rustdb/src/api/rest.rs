use axum::{
    Router,
    routing::{get, post},
    extract::{Path, State, Json},
    response::IntoResponse,
};
use super::Database;
use crate::core::{Document, Query, Result};

pub fn create_router(db: Database) -> Router {
    Router::new()
        .route("/collections/:name", get(get_collection))
        .route("/collections/:name/documents", post(create_document))
        .route("/collections/:name/query", post(query_collection))
        .with_state(db)
}

async fn get_collection(
    State(db): State<Database>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    // Utilise l'API pour accéder aux données
    db.get_collection(&name).await
}
