mod natural_selection;
mod survival_rules;

pub use natural_selection::{NaturalSelection, ExecutionResult};
pub use survival_rules::{SurvivalRules, Safety};

use std::path::Path;
use std::process::Command;
use std::time::{Instant, Duration};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ExecutionResult {
    pub survived: bool,
    pub lifetime_ms: u64,
}

pub struct Executor {
    max_runtime: Duration,
}

impl Executor {
    pub fn new(max_runtime: Duration) -> Self {
        Self { max_runtime }
    }

    pub fn execute(&self, path: &Path) -> ExecutionResult {
        let start = Instant::now();
        
        let output = Command::new("cargo")
            .arg("run")
            .current_dir(path)
            .output();

        let duration = start.elapsed();
        if duration > self.max_runtime {
            return ExecutionResult {
                survived: false,
                lifetime_ms: duration.as_millis() as u64
            };
        }

        match output {
            Ok(output) => ExecutionResult {
                survived: output.status.success(),
                lifetime_ms: duration.as_millis() as u64
            },
            Err(_) => ExecutionResult {
                survived: false,
                lifetime_ms: 0
            }
        }
    }
}
