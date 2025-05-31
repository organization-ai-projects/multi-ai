use std::fs;
use std::path::Path;
use chrono::Local;

pub fn create_backup(path: &str, file: &str) -> std::io::Result<()> {
    let source = format!("{}/{}", path, file);
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup = format!("{}/{}.{}.bak", path, file, timestamp);

    fs::copy(&source, &backup)?;
    println!("✓ Sauvegarde créée: {}", backup);
    Ok(())
}

pub fn create_backups(domain_path: &str) -> std::io::Result<()> {
    println!("📦 Création des sauvegardes...");
    
    let files = vec![
        "mod.rs",
        "trait_commands.rs",
    ];
    for file in files {
        if Path::new(&format!("{}/{}", domain_path, file)).exists() {
            create_backup(domain_path, file)?;
        }
    }

    Ok(())
}

pub fn cleanup_backups(domain_path: &str) -> std::io::Result<()> {
    let backup_pattern = format!("{}/*.bak", domain_path);
    for entry in glob::glob(&backup_pattern).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))? {
        if let Ok(path) = entry {
            fs::remove_file(path)?;
        }
    }
    println!("✓ Sauvegardes nettoyées");
    Ok(())
}
