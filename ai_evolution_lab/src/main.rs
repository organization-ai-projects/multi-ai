use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use ctrlc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use ai_evolution_lab::{
    ensure_directories,
    launch_ias,
    load_or_init_global_metadata,
    save_global_metadata,
    load_or_init_ia_list,
};

fn main() {
    let project_base = PathBuf::from("ai_evolution_lab");
    ensure_directories(&project_base);

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Arrêt en cours...");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Erreur configuration du gestionnaire Ctrl+C");

    // Charge ou initialise le metadata global
    let mut global_meta = load_or_init_global_metadata(&project_base);
    global_meta.total_runs += 1;
    global_meta.last_run = chrono::Local::now().to_rfc3339();
    save_global_metadata(&global_meta, &project_base);

    let list = load_or_init_ia_list(&project_base);
    let mut runner = launch_ias(&list);

    while running.load(Ordering::SeqCst) {
        thread::sleep(Duration::from_secs(1));
    }

    runner.stop_all();
    println!("Arrêt terminé.");
}
