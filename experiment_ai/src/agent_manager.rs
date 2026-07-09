use crate::security::SecurityManager;
use std::{fs, io, path::{Path, PathBuf}};
use std::process::{Command, Child};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng; // Ajout de rand::Rng

const TIMEOUT_SECONDS: u64 = 300; // 5 minutes pour prouver sa valeur

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub struct AgentMetadata {
    pub id: String,
    pub short_id: String,
    pub parent_id: Option<String>,
    pub generation: u32,
    pub created_at: u64,
}

#[derive(Debug)]
pub struct AgentState {
    pub path: PathBuf,
    pub id: String,
    pub process: Child,
    pub started_at: u64,  // Remplacement de SystemTime par u64
    pub successful_runs: u32,
}

#[derive(Deserialize, Serialize, bincode_next::Encode, bincode_next::Decode)]
struct Objective {
    name: String,
    description: String,
    score: i32,
    threshold: i32,
    success_count: u32,  // Changé en u32 car toujours positif
    fail_count: u32,     // Changé en u32 car toujours positif
    achieved: bool,
}

// Plus de check_objectives() car on laisse l'IA explorer librement

impl AgentState {
    pub fn new(path: PathBuf) -> io::Result<Self> {
        let id = path.file_name().unwrap().to_string_lossy().to_string();
        
        // Crée le sandbox dédié à l'agent
        fs::create_dir_all(format!("sandbox/{}", id))?;
        
        Ok(Self {
            process: Command::new("cargo")
                .arg("run")
                .current_dir(&path)
                .spawn()?,
            path: path.clone(),
            id,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            successful_runs: 0,
        })
    }

    pub fn monitor(&mut self, security: &SecurityManager) -> io::Result<Option<PathBuf>> {
        // Vérifie d'abord le timeout
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() - self.started_at;

        if elapsed > TIMEOUT_SECONDS {
            security.log_action(&format!(
                "⏰ Agent {} timeout après {}s sans résultat", 
                self.id, elapsed
            ))?;
            // Kill le process et nettoyage
            let _ = self.process.kill();
            let _ = fs::remove_dir_all(&self.path);
            return find_last_stable_agent(security);
        }

        match self.process.try_wait()? {
            Some(status) => {
                // Vérifie les fichiers dans sandbox/
                let mut valid_files = Vec::new();
                if let Ok(files) = fs::read_dir(security.get_agent_sandbox(&self.id)) {
                    for file in files.flatten() {
                        // Vérifie seulement les .rs
                        if file.path().extension().and_then(|e| e.to_str()) == Some("rs") {
                            if security.run_cargo_check(&file.path())? {
                                valid_files.push(file.path());
                                self.successful_runs += 1;
                                
                                // Si assez de succès, tente une évolution
                                if self.successful_runs > 5 {
                                    // Crée une nouvelle version
                                    let new_id = format!("{}_{}", self.id, rand::random::<u64>());
                                    let new_path = security.get_project_path(&format!("agents/{}", new_id));
                                    fs::create_dir_all(&new_path.join("src"))?;
                                    
                                    // Copie le meilleur fichier comme nouveau main.rs
                                    if let Some(best_file) = valid_files.last() {
                                        fs::copy(best_file, new_path.join("src/main.rs"))?;
                                        security.log_action(&format!(
                                            "🔄 Évolution de {} vers {}", 
                                            self.id, new_id
                                        ))?;
                                        return Ok(Some(new_path));
                                    }
                                }
                            }
                        }
                    }
                }

                if !status.success() {
                    security.log_action(&format!(
                        "❌ Agent {} échec après {} succès", 
                        self.id, self.successful_runs
                    ))?;
                    // Supprime la version instable
                    let _ = fs::remove_dir_all(&self.path);
                    find_last_stable_agent(security)
                } else {
                    Ok(None)
                }
            }
            None => Ok(None) // Agent toujours en cours
        }
    }
}

fn find_last_stable_agent(security: &SecurityManager) -> io::Result<Option<PathBuf>> {
    let agents_dir = security.get_project_path("agents");
    let mut latest = None;
    let mut latest_time = 0;

    for entry in fs::read_dir(agents_dir)?.flatten() {
        if let Ok(meta) = read_agent_metadata(&entry.path()) {
            if meta.created_at > latest_time {
                // Vérifie que la version est stable
                if security.run_cargo_check(&entry.path())? {
                    latest_time = meta.created_at;
                    latest = Some(entry.path());
                }
            }
        }
    }
    Ok(latest)
}

fn check_new_version(security: &SecurityManager, current_id: &str) -> io::Result<Option<PathBuf>> {
    let agents_dir = security.get_project_path("agents");
    for entry in fs::read_dir(agents_dir)?.flatten() {
        if let Ok(meta) = read_agent_metadata(&entry.path()) {
            if meta.parent_id.as_deref() == Some(current_id) {
                return Ok(Some(entry.path()));
            }
        }
    }
    Ok(None)
}

fn test_agent_version(security: &SecurityManager, path: &Path) -> io::Result<bool> {
    security.run_cargo_check(path)
}

pub fn read_agent_metadata(path: &Path) -> io::Result<AgentMetadata> {
    let content = fs::read_to_string(path.join("metadata.ron"))?;
    ron::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn find_latest_agent(security: &SecurityManager) -> io::Result<PathBuf> {
    let agents_dir = security.get_project_path("agents");
    let mut latest = None;
    let mut latest_time = 0;

    for entry in fs::read_dir(agents_dir)?.flatten() {
        if entry.file_type()?.is_dir() {
            if let Ok(meta) = read_agent_metadata(&entry.path()) {
                if meta.created_at > latest_time {
                    latest_time = meta.created_at;
                    latest = Some(entry.path());
                }
            }
        }
    }

    latest.ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Aucun agent trouvé"))
}
