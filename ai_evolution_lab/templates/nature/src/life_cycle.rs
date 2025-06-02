use crate::execution::{Executor, ExecutionResult};
use std::path::Path;
use std::fs;
use uuid::uuid7;

pub struct Lifecycle {
    pub environment_path: Box<Path>
}

impl Lifecycle {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            environment_path: Box::from(path.as_ref())
        }
    }

    pub fn process_candidates(&self, executor: &Executor) {
        if let Ok(entries) = fs::read_dir(&self.environment_path) {
            for entry in entries.flatten() {
                // Génère un nouveau uuid7 pour chaque forme de vie qui survit
                let result = executor.evaluate(&entry.path());
                if result.survived {
                    let new_id = uuid7();
                    let new_path = self.environment_path.join(new_id.to_string());
                    fs::rename(entry.path(), new_path).ok();
                }
                self.handle_result(&entry.path(), &result);
            }
        }
    }

    fn handle_result(&self, path: &Path, result: &ExecutionResult) {
        let result_path = path.join("result.ron");
        ron::ser::to_string(&result)
            .map(|s| fs::write(&result_path, s))
            .ok();

        if !result.survived {
            self.cleanup_failed(path);
        }
    }

    fn cleanup_failed(&self, path: &Path) {
        if path.exists() {
            fs::remove_dir_all(path).ok();
        }
    }
}
