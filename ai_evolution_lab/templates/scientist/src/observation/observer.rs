use super::types::{Observation, ObservationType};
use super::analysis::SpecimenAnalysis;
use std::path::{Path, PathBuf};
use serde::Deserialize;
use uuid::uuid7;
use std::fs;
use crate::memory::Memory;

pub struct Observer {
    environment_path: PathBuf,
    knowledge_level: u8,
}

impl Observer {
    pub fn new(environment_path: impl AsRef<Path>) -> Self {
        // ...existing new implementation...
    }

    pub fn scan_environment(&mut self) -> Vec<Observation> {
        let mut observations = Vec::new();
        
        // Parcours des formes de vie
        if let Ok(entries) = fs::read_dir(&self.environment_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        let observation = self.observe_lifeform(&entry.path());
                        observations.push(observation);
                    }
                }
            }
        }

        observations
    }

    fn observe_lifeform(&self, path: &Path) -> Observation {
        let result_file = path.join("result.ron");
        let form_file = path.join("form.ron");
        
        let is_alive = result_file.exists() && form_file.exists();
        let execution_time = self.read_execution_time(&result_file);
        let source = self.read_source_safely(path);

        // Si c'est mort et qu'on a le niveau, on dissèque
        let (source_code, was_dissected) = if !is_alive && self.knowledge_level >= 3 {
            (source.clone(), true)
        } else {
            (None, false)
        };

        // Récupération des logs s'ils existent
        let stdout = fs::read_to_string(path.join("stdout.txt")).ok();
        let stderr = fs::read_to_string(path.join("stderr.txt")).ok();

        Observation {
            specimen_id: uuid7(),
            observation_type: self.determine_observation_type(is_alive, was_dissected),
            is_alive,
            stdout,
            stderr,
            execution_time_ms: execution_time,
            source_code
        }
    }

    fn read_execution_time(&self, result_path: &Path) -> u64 {
        if let Ok(content) = fs::read_to_string(result_path) {
            if let Ok(result) = ron::from_str::<ExecutionResult>(&content) {
                return result.lifetime_ms;
            }
        }
        0
    }

    fn read_source_safely(&self, path: &Path) -> Option<String> {
        let src_path = path.join("src/main.rs");
        if src_path.exists() {
            fs::read_to_string(src_path).ok()
        } else {
            None
        }
    }

    fn seems_interesting(&self, patterns: &[String]) -> bool {
        // Plus de 3 patterns différents
        if patterns.len() > 3 {
            return true;
        }

        // Contient une structure principale
        if patterns.iter().any(|p| p == "structure_principale") {
            return true;
        }

        // Complexité intéressante
        patterns.iter()
            .any(|p| p.starts_with("niveau_imbrication_") && 
                p.chars().last().unwrap().to_digit(10).unwrap() > 2)
    }

    fn worth_dissecting(&self, observation: &Observation) -> bool {
        let mut score = 0;

        // Points pour la durée de vie
        if observation.execution_time_ms > 1000 {
            score += 2;
        }

        // Points pour la sortie
        if let Some(out) = &observation.stdout {
            score += out.lines().count() as i32;
        }

        // Points pour la complexité (si on a le code)
        if let Some(code) = &observation.source_code {
            score += code.matches('{').count() as i32;
            score += code.matches("fn").count() as i32;
        }

        // Décision finale basée sur le score et le niveau
        score > 5 || (score > 3 && self.knowledge_level >= 3)
    }

    fn observe_carefully(&self, specimen: &Path) -> Option<String> {
        let src_path = specimen.join("src/main.rs");
        
        if let Ok(code) = fs::read_to_string(src_path) {
            // On ne tue pas le spécimen, on l'observe juste attentivement
            let patterns = self.observe_surface_patterns(&code);
            let analysis = self.analyze_patterns(&patterns);
            Some(analysis)
        } else {
            None
        }
    }

    fn analyze_patterns(&self, patterns: &[String]) -> String {
        let mut insights = Vec::new();

        for pattern in patterns {
            match pattern {
                p if p.starts_with("niveau_imbrication_") => {
                    insights.push(format!("Structure imbriquée de niveau {}", 
                        p.chars().last().unwrap()));
                }
                p if p == "structure_principale" => {
                    insights.push("Possède une fonction principale".into());
                }
                _ => insights.push(format!("Motif observé: {}", pattern))
            }
        }

        insights.join("\n")
    }
}

#[derive(Deserialize)]
struct ExecutionResult {
    survived: bool,
    lifetime_ms: u64,
}

#[derive(Deserialize)]
struct LifeForm {
    total_lifetime: u64,
    // ... autres champs nécessaires
}
