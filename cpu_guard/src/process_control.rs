use std::process::Command;

#[cfg(unix)]
pub fn pause_process(pid: u32) {
    let _ = nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::Signal::SIGSTOP);
    println!("Processus mis en pause (SIGSTOP).");
}

#[cfg(unix)]
pub fn resume_process(pid: u32) {
    let _ = nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), nix::sys::signal::Signal::SIGCONT);
    println!("Processus repris (SIGCONT).");
}

#[cfg(windows)]
pub fn pause_process(pid: u32) {
    // 1. Réduire la priorité au minimum avec PowerShell
    let ps_output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Get-Process -Id {} | ForEach-Object {{ $_.PriorityClass = 'Idle'; $_.Refresh() }}", pid),
        ])
        .output();
    
    // 2. Utiliser wmic comme alternative (plus bas niveau)
    if ps_output.is_err() {
        let wmic_output = Command::new("wmic")
            .args(&[
                "process",
                "where",
                &format!("ProcessId={}", pid),
                "call",
                "setpriority",
                "idle"
            ])
            .output();
        
        match wmic_output {
            Ok(_) => println!("Processus mis en priorité minimale via WMIC."),
            Err(e) => println!("Échec de la mise en priorité minimale via WMIC : {}", e),
        }
    } else {
        println!("Processus mis en priorité minimale via PowerShell.");
    }
}

#[cfg(windows)]
pub fn resume_process(pid: u32) {
    // 1. Rétablir la priorité avec PowerShell
    let ps_output = Command::new("powershell")
        .args(&[
            "-Command",
            &format!("Get-Process -Id {} | ForEach-Object {{ $_.PriorityClass = 'Normal'; $_.Refresh() }}", pid),
        ])
        .output();
    
    // 2. Utiliser wmic comme alternative (plus bas niveau)
    if ps_output.is_err() {
        let wmic_output = Command::new("wmic")
            .args(&[
                "process",
                "where",
                &format!("ProcessId={}", pid),
                "call",
                "setpriority",
                "normal"
            ])
            .output();
        
        match wmic_output {
            Ok(_) => println!("Processus remis en priorité normale via WMIC."),
            Err(e) => println!("Échec de la remise en priorité normale via WMIC : {}", e),
        }
    } else {
        println!("Processus remis en priorité normale via PowerShell.");
    }
}

pub fn kill_process(pid: u32) -> bool {
    #[cfg(unix)]
    {
        use std::process::Command;
        let result = Command::new("kill")
            .args(&["-9", &pid.to_string()])
            .status();
        result.is_ok()
    }
    
    #[cfg(windows)]
    {
        use std::process::Command;
        let result = Command::new("taskkill")
            .args(&["/F", "/PID", &pid.to_string()])
            .status();
        result.is_ok()
    }
}