use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
struct SystemLogEntry {
    timestamp: u64,
    agent_id: String,
    action: String,
    status: String,
    details: Option<String>,
    runs: u32,
}

pub struct SecurityManager {
    project_base: String,
    allowed_dirs: Vec<String>,
}

impl SecurityManager {
    pub fn new(project_base: &str) -> Self {
        Self {
            project_base: project_base.to_string(),
            allowed_dirs: vec!["agents".to_string(), "sandbox".to_string(), 
                             "logs".to_string(), "datasets".to_string()]
        }
    }

    pub fn get_project_path(&self, subpath: &str) -> PathBuf {
        Path::new(&self.project_base).join(subpath)
    }

    pub fn get_agent_sandbox(&self, agent_id: &str) -> PathBuf {
        self.get_project_path(&format!("sandbox/{}", agent_id))
    }

    pub fn is_path_allowed<P: AsRef<Path>>(&self, path: P, agent_id: &str) -> bool {
        let abs_path = match path.as_ref().canonicalize() {
            Ok(p) => p,
            Err(_) => return false,
        };

        let root = match Path::new(&self.project_base).canonicalize() {
            Ok(r) => r,
            Err(_) => return false,
        };

        // Vérifie que c'est dans le sandbox de l'agent
        let agent_sandbox = self.get_agent_sandbox(agent_id);
        if !abs_path.starts_with(&agent_sandbox) {
            let _ = self.log_action(&format!(
                "❌ Agent {} tentative hors de son sandbox: {}", 
                agent_id, abs_path.display()
            ));
            return false;
        }

        // Au début, seul sandbox/ est autorisé en écriture
        let sandbox_dir = root.join("sandbox");
        if !abs_path.starts_with(&sandbox_dir) {
            return false;
        }

        // Vérifie qu'on est dans un dossier autorisé
        self.allowed_dirs.iter().any(|d| abs_path.starts_with(root.join(d)))
    }

    pub fn ensure_dirs(&self) -> io::Result<()> {
        for dir in &self.allowed_dirs {
            fs::create_dir_all(self.get_project_path(dir))?;
        }
        Ok(())
    }

    fn log_attempt(&self, action: &str, path: &Path, success: bool) -> io::Result<()> {
        let status = if success { "autorisé" } else { "refusé" };
        let log_entry = format!("{:?}: {} {} pour {}\n", 
            std::time::SystemTime::now(), action, status, path.display());
        
        let log_path = self.get_project_path("logs/security.log");
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path)?;
        f.write_all(log_entry.as_bytes())
    }

    pub fn safe_read_text_file<P: AsRef<Path>>(&self, path: P) -> io::Result<String> {
        // Référence pour éviter le move
        let path_ref = path.as_ref();
        
        // Pas de vérification de is_path_allowed pour la lecture
        let content = fs::read_to_string(&path_ref)?;
        
        // Log la lecture avec la référence
        self.log_action(&format!("📖 Lecture de {}", path_ref.display()))?;
        
        Ok(content)
    }

    pub fn safe_write_text_file<P: AsRef<Path>>(&self, path: P, content: &str, agent_id: &str) -> io::Result<()> {
        let path = path.as_ref();

        // Force l'écriture dans le sandbox de l'agent
        let final_path = if !path.starts_with(self.get_agent_sandbox(agent_id)) {
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown.rs");
            self.get_agent_sandbox(agent_id).join(filename)
        } else {
            path.to_path_buf()
        };

        fs::create_dir_all(final_path.parent().unwrap())?;
        let allowed = self.is_path_allowed(&final_path, agent_id);
        self.log_attempt("écriture", &final_path, allowed)?;
        
        if !allowed {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, 
                "Écriture interdite hors de sandbox/"));
        }

        fs::write(&final_path, content)?;
        self.log_action(&format!("✍️ Écriture dans sandbox: {}", final_path.display()))?;
        Ok(())
    }

    pub fn safe_move_file<P: AsRef<Path>, Q: AsRef<Path>>(&self, src: P, dst: Q) -> io::Result<()> {
        if !self.is_path_allowed(&src) || !self.is_path_allowed(&dst) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Déplacement interdit"));
        }
        fs::rename(src, dst)
    }

    pub fn run_cargo_check<P: AsRef<Path>>(&self, path: P) -> io::Result<bool> {
        if !self.is_path_allowed(&path) {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "Check interdit"));
        }
        Ok(Command::new("cargo").arg("check").current_dir(path.as_ref()).output()?.status.success())
    }

    pub fn log_action(&self, message: &str) -> io::Result<()> {
        let entry = SystemLogEntry {
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            agent_id: message.split_whitespace()
                .nth(1)
                .unwrap_or("unknown")
                .to_string(),
            action: message.split_whitespace()
                .nth(0)
                .unwrap_or("unknown")
                .to_string(),
            status: if message.contains("échoué") || message.contains("interdit") { 
                "failure" 
            } else { 
                "success" 
            }.to_string(),
            details: Some(message.to_string()),
            runs: message.split_whitespace()
                .nth(4)
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),
        };

        // Log dans system.ron
        let log_path = self.get_project_path("logs/system.ron");
        let mut entries = if log_path.exists() {
            ron::from_str(&fs::read_to_string(&log_path)?)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        } else {
            vec![]
        };
        
        entries.push(entry);
        fs::write(log_path, ron::ser::to_string_pretty(&entries, Default::default())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?)?;

        // Log aussi dans security.log pour traçabilité
        let security_path = self.get_project_path("logs/security.log");
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(security_path)?;
        writeln!(f, "{:?}: {}", SystemTime::now(), message)?;

        Ok(())
    }
}
