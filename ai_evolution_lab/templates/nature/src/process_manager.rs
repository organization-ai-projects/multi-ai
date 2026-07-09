use crate::life_form::LifeForm;
use crate::execution::Executor;
use crate::environment::FileSystem;
use std::collections::HashMap;
use std::process::{Command, Child};

pub struct ProcessManager {
    active_processes: HashMap<String, Child>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            active_processes: HashMap::new(),
        }
    }

    pub fn launch_life_form(&mut self, id: &str, fs: &FileSystem) {
        let form_path = fs.get_form_path(id);
        if let Ok(child) = Command::new("cargo")
            .args(["run", "--manifest-path"])
            .arg(form_path.join("Cargo.toml"))
            .spawn()
        {
            self.active_processes.insert(id.to_string(), child);
        }
    }

    pub fn check_active_processes(&mut self, population: &mut crate::environment::Population, executor: &Executor) {
        self.active_processes.retain(|id, process| {
            match process.try_wait() {
                Ok(Some(status)) => {
                    if let Some(form) = population.get_form_mut(id) {
                        form.update_state(crate::execution::ExecutionResult {
                            survived: status.success(),
                            lifetime_ms: 0,
                        });
                    }
                    false
                }
                Ok(None) => true,
                Err(_) => false,
            }
        });
    }

    pub fn clean_shutdown(&mut self) {
        for (_id, mut process) in self.active_processes.drain() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}
