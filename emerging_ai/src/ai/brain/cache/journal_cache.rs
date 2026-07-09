use crate::brain::persistent::journal::Journal;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use uuid::Uuid;

pub struct JournalCache {
    journal: Journal,
    dirty: bool,
}

impl JournalCache {
    pub fn new() -> Self {
        Self {
            journal: Journal::new(),
            dirty: false,
        }
    }

    pub fn add_entry(&mut self, action: &str, entity_type: &str, uuid: &str) {
        self.journal.add_entry(action, entity_type, uuid);
        self.dirty = true;
    }

    pub fn mark_completed(&mut self, uuid: &str) {
        self.journal.mark_completed(uuid);
        self.dirty = true;
    }

    pub fn list(&self) -> Vec<Uuid> {
        self.journal
            .entries
            .iter()
            .map(|entry| Uuid::parse_str(&entry.uuid).unwrap())
            .collect()
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&JournalEntry> {
        self.journal
            .entries
            .iter()
            .find(|entry| entry.uuid == uuid.to_string())
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if self.dirty {
            self.journal.save_to_file(Path::new("journal.ron"))?;
            self.dirty = false;
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.journal.entries.clear();
        self.dirty = false;
    }
}
