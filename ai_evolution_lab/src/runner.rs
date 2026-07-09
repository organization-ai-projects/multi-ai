use std::process::{Command, Child};
use std::collections::HashMap;
use crate::models::IaList;

pub struct Runner {
    processes: HashMap<String, Child>
}

impl Runner {
    pub fn new() -> Self {
        Self {
            processes: HashMap::new()
        }
    }

    pub fn launch_ias(&mut self, list: &IaList) {
        for path in list.nature.iter().chain(&list.scientist).chain(&list.life_form) {
            if let Ok(child) = Command::new("cargo")
                .args([
                    "run", "--manifest-path",
                    &format!("{}/Cargo.toml", path)
                ])
                .spawn() 
            {
                self.processes.insert(path.clone(), child);
            }
        }
    }

    pub fn stop_all(&mut self) {
        for (_path, mut process) in self.processes.drain() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}

pub fn launch_ias(list: &IaList) -> Runner {
    let mut runner = Runner::new();
    runner.launch_ias(list);
    runner
}
