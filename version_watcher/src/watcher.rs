use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config, EventKind};
use std::sync::mpsc::channel;
use std::time::Duration;
use crate::store::create_snapshot;
use std::path::Path;

pub fn start_watching(path: &str) {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(Path::new(path), RecursiveMode::Recursive).unwrap(); // Conversion explicite de `&str` en `&Path`

    println!("🔍 Watching '{}'", path);

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                if matches!(event.unwrap().kind, EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)) {
                    println!("⚡ Change detected. Creating snapshot...");
                    create_snapshot(path);
                }
            }
            Err(_) => {}
        }
    }
}
