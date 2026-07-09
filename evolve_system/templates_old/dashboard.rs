// ... dans ton dashboard.rs

// (Ajoute un endpoint historique)
.route("/api/history", get(get_history))

async fn get_history(
    State(state): State<Arc<RwLock<DashboardState>>>
) -> Json<Vec<DashboardState>> {
    // Charge les stats des générations sauvegardées dans un dossier, ou conserve-les en RAM
    // (exemple simplifié :)
    let history_files = std::fs::read_dir("history").unwrap();
    let mut history = vec![];
    for f in history_files.filter_map(Result::ok) {
        let stat = std::fs::read_to_string(f.path()).ok().and_then(|s| serde_json::from_str(&s).ok());
        if let Some(s) = stat { history.push(s); }
    }
    Json(history)
}
