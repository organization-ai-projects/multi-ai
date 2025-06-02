use std::path::Path;
use std::fs;
use std::thread;
use std::time::Duration;

use ai_evolution_lab::{
    ensure_directories,
    launch_ias,
    load_or_init_global_metadata,
    save_global_metadata,
    load_or_init_ia_list,
};

fn main() {
    ensure_directories();

    // Charge ou initialise le metadata global
    let mut global_meta = load_or_init_global_metadata();
    global_meta.total_runs += 1;
    global_meta.last_run = chrono::Local::now().to_rfc3339();
    save_global_metadata(&global_meta);

    let list = load_or_init_ia_list();
    launch_ias(&list);

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
