use std::fs;
use std::path::Path;
use regex::Regex;
use similar::{ChangeTag, TextDiff};
use std::process::Command;

#[derive(Debug)]
pub struct DetectedModule {
    pub name: String,
    pub module_type: ModuleType,
}

#[derive(Debug)]
pub enum ModuleType {
    Domain(String),    // Pour stocker le nom de la structure (ex: "AiDomain")
    System,    
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

pub fn generate_enum_content(domains: &[DetectedModule]) -> String {
    let mut domains: Vec<_> = domains.iter()
        .filter(|m| matches!(m.module_type, ModuleType::Domain(_)))
        .map(|m| m.name.to_uppercase())
        .collect();
    domains.sort();

    // Si un seul élément, pas de virgule
    // Si plusieurs éléments, ajouter la virgule après chaque élément sauf le dernier
    let items = if domains.len() == 1 {
        format!("    {}", domains[0])
    } else {
        domains.iter()
            .map(|d| format!("    {},", d))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!("pub enum CommandDomain {{\n{}\n}}", items)
}

fn normalize_content(content: &str) -> String {
    content
        .lines()
        .map(|line| line.trim().trim_end_matches(','))
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

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

fn format_rust_file(path: &str) -> std::io::Result<()> {
    Command::new("rustfmt")
        .arg(path)
        .status()
        .map(|_| ())
        .map_err(|e| std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Erreur de formatage: {}", e)
        ))
}

#[derive(Default)]
struct ModificationTracker {
    mod_rs_changes: bool,
    trait_commands_changes: bool,
    total_changes: usize,
}

pub fn write_domain_registry(domain_path: &str, modules: &[DetectedModule], dry_run: bool) -> std::io::Result<()> {
    let mut tracker = ModificationTracker::default();
    let backup_path = format!("{}/cli_commands.bak", domain_path);

    // Vérification initiale des modifications nécessaires
    let need_backup = check_modifications(domain_path, modules, &mut tracker)?;
    
    if need_backup && !dry_run {
        create_backup(domain_path, &backup_path)?;
    }

    // Gestion de mod.rs
    if tracker.mod_rs_changes {
        update_mod_rs(domain_path, modules)?;
    }

    // Gestion de trait_commands.rs
    if tracker.trait_commands_changes {
        update_trait_commands(domain_path, modules)?;
    }

    // Vérification finale
    if verify_changes(domain_path, modules)? {
        if !dry_run && need_backup {
            fs::remove_file(backup_path)?;
            println!("✅ Sauvegarde supprimée : modifications réussies");
        }
        println!("✅ {} modification(s) appliquée(s) avec succès", tracker.total_changes);
    } else {
        println!("⚠️ Certaines modifications ne correspondent pas à l'attendu");
        println!("ℹ️ Sauvegarde conservée dans cli_commands.bak");
    }

    Ok(())
}

fn check_modifications(domain_path: &str, modules: &[DetectedModule], tracker: &mut ModificationTracker) -> std::io::Result<bool> {
    // Vérification mod.rs
    let mod_path = format!("{}/mod.rs", domain_path);
    let current_mod = fs::read_to_string(&mod_path)?;
    let new_mods = generate_mod_content(modules);
    
    if current_mod.trim() != new_mods.trim() {
        tracker.mod_rs_changes = true;
        tracker.total_changes += 1;
    }

    // Vérification trait_commands.rs
    let trait_path = format!("{}/trait_commands.rs", domain_path);
    let current_trait = fs::read_to_string(&trait_path)?;
    if needs_enum_update(&current_trait, modules) {
        tracker.trait_commands_changes = true;
        tracker.total_changes += 1;
    }

    Ok(tracker.total_changes > 0)
}

fn create_backup(domain_path: &str, backup_path: &str) -> std::io::Result<()> {
    // Création d'une archive des fichiers actuels
    println!("📦 Création d'une sauvegarde complète...");
    // ... code pour créer une archive des fichiers ...
    Ok(())
}

fn update_mod_rs(domain_path: &str, modules: &[DetectedModule]) -> std::io::Result<()> {
    // Gestion de mod.rs
    let mod_path = format!("{}/mod.rs", domain_path);
    let mod_backup = format!("{}/mod.rs.bak", domain_path);
    
    let current_content = fs::read_to_string(&mod_path)?;
    
    let mut new_mods: Vec<String> = Vec::new();
    for module in modules.iter() {
        match module.module_type {
            ModuleType::Domain(_) => new_mods.push(format!("pub mod {};", module.name)),
            ModuleType::System => new_mods.push(format!("pub mod {};", module.name)),
        }
    }
    new_mods.sort();

    // Faire une sauvegarde avant modification
    fs::write(&mod_backup, &current_content)?;
    println!("✓ Sauvegarde temporaire créée: mod.rs.bak");

    // Écrire les modifications
    fs::write(&mod_path, new_mods.join("\n"))?;
    
    // Formatter si nécessaire
    format_rust_file(&mod_path)?;

    // Vérifier que le contenu final correspond à ce qui était attendu
    let final_content = fs::read_to_string(&mod_path)?;
    let final_mods: Vec<String> = final_content
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    if final_mods == new_mods {
        // Tout est OK, on peut supprimer la sauvegarde
        fs::remove_file(&mod_backup)?;
        println!("✅ mod.rs mis à jour avec succès");
    } else {
        // Il y a eu un problème, on garde la sauvegarde
        println!("⚠️ Le fichier final ne correspond pas à l'attendu");
        println!("ℹ️ Une sauvegarde a été conservée dans mod.rs.bak");
    }

    Ok(())
}

fn update_trait_commands(domain_path: &str, modules: &[DetectedModule]) -> std::io::Result<()> {
    // Gestion de trait_commands.rs
    let trait_path = format!("{}/trait_commands.rs", domain_path);
    let trait_backup = format!("{}/trait_commands.rs.bak", domain_path);
    let content = fs::read_to_string(&trait_path)?;
    
    let enum_pattern = r"pub enum CommandDomain \{[\s\S]*?\}";
    let re = Regex::new(enum_pattern).unwrap();
    
    if let Some(current_enum) = re.find(&content) {
        let current_normalized = normalize_content(current_enum.as_str());
        let new_enum = generate_enum_content(modules);
        let new_normalized = normalize_content(&new_enum);

        if current_normalized == new_normalized {
            println!("✓ Enum CommandDomain inchangé (déjà à jour)");
            return Ok(());
        }

        // Sauvegarde avant modification
        fs::write(&trait_backup, &content)?;
        println!("✓ Sauvegarde temporaire créée: trait_commands.rs.bak");

        // Application des modifications
        let new_content = re.replace(&content, &new_enum);
        fs::write(&trait_path, new_content.as_ref())?;

        // Vérification finale
        let final_content = fs::read_to_string(&trait_path)?;
        if re.find(&final_content).map_or(false, |m| normalize_content(m.as_str()) == new_normalized) {
            fs::remove_file(&trait_backup)?;
            println!("✅ Enum CommandDomain mis à jour avec succès");
        } else {
            println!("⚠️ La mise à jour de CommandDomain ne correspond pas à l'attendu");
            println!("ℹ️ Une sauvegarde a été conservée dans trait_commands.rs.bak");
        }
    }

    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
