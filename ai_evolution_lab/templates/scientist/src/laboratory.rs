use std::path::{Path, PathBuf};
use uuid::uuid7;

pub struct Laboratory {
    specimens: Vec<CapturedSpecimen>,
    max_specimens: usize,
}

pub struct CapturedSpecimen {
    original_id: String,
    capture_id: String,
    capture_location: String,
    original_lifetime: u64,
    laboratory_tests: Vec<TestResult>,
}

impl Laboratory {
    pub fn new(path: &PathBuf) -> Self {
        Self {
            specimens: Vec::new(),
            max_specimens: 100,
        }
    }

    pub fn capture_specimen(&mut self, env_path: &Path, specimen_id: &str) -> Option<String> {
        // Tente de capturer un spécimen vivant
        let source = env_path.join(specimen_id);
        if !source.exists() {
            eprintln!("Le spécimen {} n'existe pas dans {}", specimen_id, env_path.display());
            return None;
        }

        let capture_id = uuid::Uuid::new_v7().to_string();
        let target = Path::new("laboratory/specimens").join(&capture_id);

        // Copie le spécimen (ne le tue pas dans la nature)
        if std::fs::copy_dir_all(&source, &target).is_ok() {
            let specimen = CapturedSpecimen {
                original_id: specimen_id.to_string(),
                capture_id: capture_id.clone(),
                capture_location: source.to_string_lossy().to_string(),
                original_lifetime: 0, // à remplir
                laboratory_tests: Vec::new(),
            };
            
            self.specimens.push(specimen);
            Some(capture_id)
        } else {
            None
        }
    }
}
