use crate::laboratory::Laboratory;
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

pub struct EnvironmentScanner {
    environment_path: PathBuf,
}

impl EnvironmentScanner {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            environment_path: PathBuf::from(path.as_ref()),
        }
    }

    pub fn scan(&self) -> Vec<PathBuf> {
        let mut specimens = Vec::new();

        if let Ok(entries) = fs::read_dir(&self.environment_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        specimens.push(entry.path());
                    }
                }
            }
        }

        specimens
    }

    pub fn get_specimen_files(&self, path: &Path) -> Option<SpecimenFiles> {
        Some(SpecimenFiles {
            form: self.read_if_exists(path.join("form.ron"))?,
            result: self.read_if_exists(path.join("result.ron")),
            source: self.read_if_exists(path.join("src/main.rs")),
            stdout: self.read_if_exists(path.join("stdout.txt")),
            stderr: self.read_if_exists(path.join("stderr.txt")),
        })
    }

    pub fn scan_and_capture(&self, lab: &mut Laboratory) {
        let specimens = self.scan();
        info!("Détecté {} spécimens potentiels", specimens.len());

        for path in specimens {
            if let Some(files) = self.get_specimen_files(&path) {
                if self.is_interesting(&files) {
                    info!("Tentative de capture du spécimen {}", path.display());
                    if let Some(id) = lab.capture_specimen(&path) {
                        info!("Spécimen capturé avec succès: {}", id);
                    }
                }
            }
        }
    }

    pub fn scan_and_analyze(&mut self, lab: &mut Laboratory) -> Vec<Specimen> {
        let mut specimens = Vec::new();

        for path in self.scan() {
            if let Some(files) = self.get_specimen_files(&path) {
                if self.is_interesting(&files) {
                    info!("Spécimen intéressant détecté: {}", path.display());

                    if let Some(captured_id) = lab.capture_specimen(&path) {
                        info!("Spécimen capturé avec succès: {}", captured_id);
                        if let Some(specimen) = lab.get_specimen(&captured_id) {
                            specimens.push(specimen.clone());
                        }
                    }
                }
            }
        }

        specimens
    }

    fn read_if_exists(&self, path: PathBuf) -> Option<String> {
        fs::read_to_string(path).ok()
    }

    fn is_interesting(&self, files: &SpecimenFiles) -> bool {
        // Critères pour déterminer si un spécimen mérite d'être capturé
        if let Some(source) = &files.source {
            // Analyse rapide du code source
            source.lines().count() > 5
        } else {
            false
        }
    }
}

pub struct SpecimenFiles {
    pub form: String,      // Obligatoire
    pub result: Option<String>,
    pub source: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
