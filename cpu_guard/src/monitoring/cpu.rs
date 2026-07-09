use sysinfo::{System, Pid};

pub fn check_cpu_usage(system: &mut System, pid: u32, threshold: f32) -> (bool, f32) {
    let mut cpu_overload = false;
    let mut process_cpu = 0.0;
    
    // Vérifier le CPU de l'IA spécifique
    if let Some(proc) = system.process(Pid::from_u32(pid)) {
        process_cpu = proc.cpu_usage();
        if process_cpu > threshold {
            cpu_overload = true;
        }
    }
    
    // Vérifier le CPU global du système si pas déjà en surcharge
    if !cpu_overload {
        let global_cpu = system.global_cpu_info().cpu_usage();
        if global_cpu > threshold {
            cpu_overload = true;
            process_cpu = global_cpu; // Pour les logs
        }
    }
    
    (cpu_overload, process_cpu)
}

/// Vérifie si l'utilisation du CPU a dépassé le seuil pendant une période prolongée
pub fn is_cpu_overload_sustained(system: &mut System, pid: u32, threshold: f32, check_count: usize) -> bool {
    let mut overload_count = 0;
    
    for _ in 0..check_count {
        let (overload, _) = check_cpu_usage(system, pid, threshold);
        if overload {
            overload_count += 1;
        }
        
        std::thread::sleep(std::time::Duration::from_millis(200));
        system.refresh_all();
    }
    
    // On considère une surcharge soutenue si au moins la moitié des vérifications sont positives
    overload_count >= check_count / 2
}
