use super::types::{DetectedModule, ModuleType};
use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn show_diff(old: &str, new: &str, file_name: &str) {
    let diff = TextDiff::from_lines(old.trim(), new.trim());
    
    println!("\n  ⚠️ Modifications nécessaires dans {} :\n", file_name);
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => println!("  \x1b[31m- {}\x1b[0m", change.to_string().trim()),
            ChangeTag::Insert => println!("  \x1b[32m+ {}\x1b[0m", change.to_string().trim()),
            ChangeTag::Equal => println!("    {}", change.to_string().trim()),
        }
    }
    println!();
}

pub fn format_rust_file(path: &str) -> std::io::Result<()> {
    Command::new("rustfmt")
        .arg(path)
        .status()
        .map(|_| ())
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Erreur de formatage: {}", e)
        ))
}

pub fn normalize_content(content: &str) -> String {
    content
        .lines()
        .map(|line| line.trim().trim_end_matches(','))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn collect_domains(domain_path: &str) -> std::io::Result<Vec<DetectedModule>> {
    let path = Path::new(domain_path);
    if !path.exists() {
        return Ok(Vec::new());
    }

    let mut modules = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        if entry.file_type()?.is_dir() {
            let struct_name = format!("{}Domain", capitalize(&name));
            modules.push(DetectedModule {
                name: name.clone(),
                module_type: ModuleType::Domain(struct_name),
            });
        } else if name.ends_with(".rs") && name != "mod.rs" {
            modules.push(DetectedModule {
                name: name.trim_end_matches(".rs").to_string(),
                module_type: ModuleType::System,
            });
        }
    }
    Ok(modules)
}
