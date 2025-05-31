use super::types::*;
use std::fs;

pub fn check_modifications(
    domain_path: &str,
    mod_rs_content: &str,
    trait_enum_content: &str,
    tracker: &mut ModificationTracker,
) -> std::io::Result<bool> {
    // Vérifier mod.rs
    let current = fs::read_to_string(&format!("{}/mod.rs", domain_path))?;
    if current.trim() != mod_rs_content.trim() {
        tracker.mod_rs_changes = true;
        tracker.total_changes += 1;
    }

    // Vérifier trait_commands.rs
    let trait_content = fs::read_to_string(&format!("{}/trait_commands.rs", domain_path))?;
    if !trait_content.contains(trait_enum_content) {
        tracker.trait_commands_changes = true;
        tracker.total_changes += 1;
    }

    Ok(tracker.total_changes > 0)
}

pub fn apply_changes(
    domain_path: &str,
    tracker: &ModificationTracker,
    mod_rs_content: Option<&str>,
    trait_enum_content: Option<&str>,
) -> std::io::Result<bool> {
    let mut success = true;

    if tracker.mod_rs_changes {
        if let Some(content) = mod_rs_content {
            success &= update_file(&format!("{}/mod.rs", domain_path), content)?;
        }
    }

    if tracker.trait_commands_changes {
        if let Some(enum_content) = trait_enum_content {
            success &= update_enum_in_file(&format!("{}/trait_commands.rs", domain_path), enum_content)?;
        }
    }

    Ok(success)
}

fn update_file(path: &str, content: &str) -> std::io::Result<bool> {
    fs::write(path, content)?;
    super::utils::format_rust_file(path)?;
    Ok(true)
}

fn update_enum_in_file(path: &str, new_enum: &str) -> std::io::Result<bool> {
    let content = fs::read_to_string(path)?;
    let re = regex::Regex::new(r"pub enum CommandDomain \{[\s\S]*?\}").unwrap();
    let new_content = re.replace(&content, new_enum);
    fs::write(path, new_content.as_ref())?;
    Ok(true)
}
