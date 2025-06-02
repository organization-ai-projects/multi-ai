use crate::survival_rules::SurvivalRules;
use crate::lifecycle::LifeForm;
use std::path::Path;
use std::process::Command;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExecutionResult {
    pub survived: bool,
    pub lifetime_ms: u64,
}

pub struct NaturalSelection {
    pub survival_rules: SurvivalRules,
    pub max_runtime: Duration
}

impl NaturalSelection {
    pub fn new() -> Self {
        Self {
            survival_rules: SurvivalRules::new(),
            max_runtime: Duration::from_secs(5)
        }
    }

    pub fn evaluate(&self, form: &LifeForm) -> ExecutionResult {
        let path = Path::new("environment/forms").join(&form.id);
        
        // Vérifie si le code peut survivre
        if !self.assess_viability(&form.source_code) {
            return ExecutionResult { survived: false, lifetime_ms: 0 };
        }

        // Exécute et mesure
        let start = Instant::now();
        let output = Command::new("cargo")
            .arg("run")
            .current_dir(&path)
            .output();

        let lifetime = start.elapsed().as_millis() as u64;
        
        match output {
            Ok(output) => ExecutionResult {
                survived: output.status.success(),
                lifetime_ms: lifetime
            },
            Err(_) => ExecutionResult {
                survived: false,
                lifetime_ms: 0
            }
        }
    }

    fn assess_viability(&self, genetic_code: &str) -> bool {
        // Vérifie uniquement les motifs létaux
        !self.survival_rules.contains_lethal_code(genetic_code)
    }
}
