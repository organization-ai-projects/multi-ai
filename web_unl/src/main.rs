use axum::{
    extract::{Path, Multipart},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Router,
};
use std::{fs, net::SocketAddr, path::PathBuf};
use tokio::fs as tokio_fs;
use tokio::net::TcpListener;

// Fonction pour obtenir le chemin du dossier storage
fn storage_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("storage");
    path
}

#[tokio::main]
async fn main() {
    // Utiliser un chemin relatif au projet pour storage
    let storage_dir = storage_path();
    if !storage_dir.exists() {
        fs::create_dir_all(&storage_dir).expect("Impossible de créer le dossier storage");
        println!("📁 Dossier storage créé à {}", storage_dir.display());
    } else {
        println!("📁 Dossier storage existant à {}", storage_dir.display());
    }

    let app = Router::new()
        .route("/", get(home))
        .route("/project/:name", get(get_project))
        .route("/upload", post(upload_project))
        .route("/search", get(search));

    // Utiliser un port différent (8081) si 8080 est déjà utilisé
    let mut port = 8080;
    let mut addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    // Essayer d'autres ports si nécessaire
    let listener = loop {
        match TcpListener::bind(addr).await {
            Ok(listener) => break listener,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AddrInUse {
                    port += 1;
                    addr = SocketAddr::from(([0, 0, 0, 0], port));
                    println!("Port {} déjà utilisé, tentative avec le port {}", port-1, port);
                } else {
                    panic!("Erreur lors de la liaison au port: {}", e);
                }
            }
        }
    };
    
    println!("🌐 web_unl running at http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<&'static str> {
    Html(r#"<h1>Serveur UNL</h1>
        <p>Ce serveur héberge des fichiers au format .unl pour le navigateur universal.</p>
        <p>Pour accéder à un projet dans le navigateur universal: <code>unl://nom_du_projet</code></p>
        <p>Pour accéder directement depuis un navigateur web: <code>http://localhost:8080/project/nom_du_projet</code></p>
        <p>Pour rechercher: <code>http://localhost:8080/search?q=terme</code></p>
        <p>Pour téléverser: Utilisez une requête POST multipart à <code>http://localhost:8080/upload</code></p>"#)
}

async fn get_project(Path(name): Path<String>) -> Result<String, StatusCode> {
    let path = storage_path().join(format!("{}.unl", name));
    fs::read_to_string(path).map_err(|_| StatusCode::NOT_FOUND)
}

async fn upload_project(mut multipart: Multipart) -> Result<String, StatusCode> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap_or("unknown.unl").to_string();
        let data = field.bytes().await.unwrap();
        let path = storage_path().join(&file_name);
        tokio_fs::write(&path, &data).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        println!("✅ Uploaded: {}", file_name);
        return Ok(format!("Uploaded: {}", file_name));
    }
    Err(StatusCode::BAD_REQUEST)
}

async fn search(axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>) -> Html<String> {
    let query = params.get("q").cloned().unwrap_or_default().to_lowercase();
    let entries = match fs::read_dir(storage_path()) {
        Ok(entries) => entries,
        Err(_) => return Html("<h2>Erreur de lecture du dossier storage</h2>".to_string()),
    };
    let mut results = vec![];

    for entry in entries {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.to_lowercase().contains(&query) {
            results.push(name);
        }
    }

    let list = results.iter().map(|r| format!("<li>{}</li>", r)).collect::<String>();
    Html(format!("<h2>Search results</h2><ul>{}</ul>", list))
}
