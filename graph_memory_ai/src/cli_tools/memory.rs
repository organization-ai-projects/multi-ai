use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub fn get_cli_tools_dir() -> PathBuf {
    let mut dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    
    // Chercher le dossier graph_memory_ai/cli_tools en remontant
    while !dir.ends_with("graph_memory_ai") {
        if !dir.pop() {
            dir = PathBuf::from("graph_memory_ai");
            break;
        }
    }
    
    let cli_tools = dir.join("cli_tools");
    println!("Chemin cli_tools complet: {:?}", cli_tools);
    cli_tools
}

pub fn list_cli_tools() -> Vec<String> {
    let tools_dir = get_cli_tools_dir();
    println!("Recherche dans : {:?}", tools_dir);
    
    fs::read_dir(&tools_dir)
        .map(|entries| {
            entries
                .filter_map(|entry| {
                    entry.ok().and_then(|e| {
                        let path = e.path();
                        if path.is_file() && is_executable(&path) {
                            path.file_name()?.to_str().map(String::from)
                        } else {
                            None
                        }
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn run_cli_tool(tool_path: &str, args: &[&str]) -> Result<Output, String> {
    Command::new(tool_path)
        .args(args)
        .output()
        .map_err(|e| format!("Erreur lancement {}: {}", tool_path, e))
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
    }
    #[cfg(windows)]
    {
        path.extension()
            .map(|ext| ext == "exe" || ext == "bat" || ext == "cmd")
            .unwrap_or(false)
    }
}
