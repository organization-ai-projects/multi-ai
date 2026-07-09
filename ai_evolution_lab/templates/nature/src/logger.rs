use serde::{Serialize, Deserialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: i64,
    pub event: String,
    pub zoom_level: u8, // Niveau de zoom requis pour voir cet événement
}

pub struct NatureLogger {
    log_path: PathBuf,
}

impl NatureLogger {
    pub fn new(log_path: PathBuf) -> Self {
        Self { log_path }
    }

    pub fn log_event(&self, id: &str, entry: LogEntry) {
        let log_file = self.log_path.join(format!("{}.log", id));
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file) {
            if let Ok(serialized) = ron::ser::to_string(&entry) {
                let _ = writeln!(file, "{}", serialized);
            }
        }
    }

    pub fn get_logs(&self, id: &str, zoom_level: u8) -> Vec<LogEntry> {
        let log_file = self.log_path.join(format!("{}.log", id));
        if let Ok(content) = fs::read_to_string(log_file) {
            content
                .lines()
                .filter_map(|line| ron::from_str::<LogEntry>(line).ok())
                .filter(|entry| entry.zoom_level <= zoom_level)
                .collect()
        } else {
            Vec::new()
        }
    }
}
