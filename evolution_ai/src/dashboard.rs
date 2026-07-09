use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json; // Ajout de l'import
use std::{collections::HashMap, fs, path::Path, sync::Arc};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::{
    ai_manager::AIManager,
    evolution::Evolution,
    monitoring::EvolutionSnapshot,
    species::{SpeciesStats, SpeciesType},
};

#[derive(Clone, Serialize, Default)] // Ajout de Default
pub struct DashboardState {
    pub current_generation: usize,
    pub species_stats: HashMap<SpeciesType, SpeciesStats>,
    pub best_performers: Vec<(String, f64)>,
    pub global_diversity: f64,
}

#[derive(Clone)]
pub struct Dashboard {
    state: Arc<RwLock<DashboardState>>,
    evolution: Arc<RwLock<Evolution>>,
    manager: Arc<AIManager>,
}

impl Dashboard {
    pub fn new(evolution: Evolution, manager: AIManager) -> Self {
        Self {
            state: Arc::new(RwLock::new(DashboardState::default())),
            evolution: Arc::new(RwLock::new(evolution)),
            manager: Arc::new(manager),
        }
    }

    pub fn update(&self, snapshot: &EvolutionSnapshot) {
        // Cloner les données nécessaires avant de les déplacer dans la tâche
        let generation = snapshot.generation;
        let species_stats = snapshot.species_stats.clone();
        let global_diversity = snapshot.global_diversity;

        tokio::spawn({
            let state = self.state.clone();
            async move {
                let mut state = state.write().await;
                state.current_generation = generation;
                state.species_stats = species_stats;
                state.global_diversity = global_diversity;
            }
        });
    }

    pub async fn run_server(self) {
        let state = self.state.clone();
        let evolution = self.evolution.clone();
        let manager = self.manager.clone();

        let app = Router::new()
            .route("/api/stats", get(get_stats))
            .route("/api/force-mutation", post(force_mutation))
            .route("/api/reset", post(reset_population))
            .route("/api/save-generation", post(save_generation))
            .route("/api/control/species-boost", post(boost_species))
            .nest_service("/", ServeDir::new("dashboard/static"))
            .layer(CorsLayer::permissive())
            .with_state(AppState {
                dashboard: state,
                evolution,
                manager,
            });

        let addr = "0.0.0.0:3000";
        println!("Démarrage du dashboard sur http://localhost:3000");

        match axum::Server::bind(&addr.parse().unwrap())
            .serve(app.into_make_service())
            .await
        {
            Ok(_) => println!("Dashboard arrêté"),
            Err(e) => eprintln!("Erreur dashboard: {}", e),
        }
    }
}

#[derive(Clone)]
struct AppState {
    dashboard: Arc<RwLock<DashboardState>>,
    evolution: Arc<RwLock<Evolution>>,
    manager: Arc<AIManager>,
}

async fn get_stats(State(state): State<AppState>) -> Json<DashboardState> {
    Json(state.dashboard.read().await.clone())
}

async fn force_mutation(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut evolution = state.evolution.write().await;
    let result = evolution.force_random_mutation(&state.manager);

    Json(json!({
        "status": if result.is_ok() { "success" } else { "error" },
        "message": result.map_or_else(
            |e| e.to_string(),
            |id| format!("Mutation forcée sur AI_{:05}", id)
        )
    }))
}

async fn reset_population(State(state): State<AppState>) -> Json<serde_json::Value> {
    let result = state.manager.reset_population();

    Json(json!({
        "status": if result.is_ok() { "success" } else { "error" },
        "message": result.map_or_else(
            |e| e.to_string(),
            |_| "Population réinitialisée".to_string()
        )
    }))
}

async fn save_generation(State(state): State<AppState>) -> Json<serde_json::Value> {
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let save_dir = format!("saves/generation_{}", timestamp);

    if let Err(e) = fs::create_dir_all(&save_dir) {
        return Json(json!({
            "status": "error",
            "message": format!("Erreur création dossier: {}", e)
        }));
    }

    // On utilise directement le manager maintenant
    for ai_path in state.manager.get_all_ais() {
        let dest = format!(
            "{}/{}",
            save_dir,
            ai_path.file_name().unwrap().to_str().unwrap()
        );
        if let Err(e) = fs::copy(ai_path, dest) {
            return Json(json!({
                "status": "error",
                "message": format!("Erreur copie: {}", e)
            }));
        }
    }

    Json(json!({
        "status": "success",
        "path": save_dir
    }))
}

#[derive(Deserialize)]
struct BoostSpeciesRequest {
    species: String,
    boost_factor: f64,
}

async fn boost_species(
    State(state): State<AppState>,
    Json(payload): Json<BoostSpeciesRequest>,
) -> Json<serde_json::Value> {
    let mut evolution = state.evolution.write().await;
    evolution.boost_species_mutation_rate(&payload.species, payload.boost_factor);

    Json(json!({
        "status": "success",
        "message": format!("Boost appliqué à l'espèce {}", payload.species)
    }))
}
