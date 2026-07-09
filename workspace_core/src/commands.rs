use crate::scanner;
use crate::workspace::config::WorkspaceConfig;
use reqwest::Client;
use std::path::Path;

pub struct RustDbClient {
    client: Client,
    base_url: String,
}

impl RustDbClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: url.to_string(),
        }
    }

    pub async fn create_collection(&self, name: &str) -> Result<(), String> {
        self.client
            .post(&format!("{}/collections", self.base_url))
            .json(&name)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn insert_document<T: serde::Serialize>(
        &self,
        collection: &str,
        doc: T,
    ) -> Result<(), String> {
        self.client
            .post(&format!(
                "{}/collections/{}/documents",
                self.base_url, collection
            ))
            .json(&doc)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub async fn init_workspace(name: String) -> Result<(), String> {
    let db = RustDbClient::new("http://localhost:8080");
    db.create_collection(&name).await?;
    Ok(())
}

pub async fn scan_workspace(name: &str) -> Result<(), String> {
    let db = RustDbClient::new("http://localhost:8080");
    let config = WorkspaceConfig::default();
    let projects = scanner::scan_projects(Path::new(&config.scan_paths[0]), &config.excluded_paths);

    for project in projects {
        db.insert_document(name, project.data).await?;
    }

    // Note: Counting documents would require a separate endpoint in the RustDB API
    println!("✅ {} projets trouvés et sauvegardés", projects.len());
    Ok(())
}
