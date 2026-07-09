use crate::version::{SemVer, VersionSnapshot};
use crate::loader::get_latest_impact;
use std::fs::{write, create_dir_all};
use toml_edit::{Document, value};
use crate::loader::Snapshot;
use serde::{Serialize, Deserialize};
use crate::git::GitRepository;

#[derive(Debug, Clone, Serialize, Deserialize)] // Ajout de `Deserialize`
pub enum Impact {
    Patch,
    Minor,
    Major,
}

pub fn bump_from_snapshots(repo: &GitRepository) -> String {
    let snapshots = repo.get_snapshots();
    let latest_snapshot = snapshots.last().expect("Aucun snapshot trouvé");

    let new_version = match latest_snapshot.impact {
        Impact::Major => repo.bump_major(),
        Impact::Minor => repo.bump_minor(),
        Impact::Patch => repo.bump_patch(),
    };

    new_version
}

pub fn generate_changelog(version: &str, impact: &Impact, files: &[String]) -> String {
    let title = match impact {
        Impact::Major => "🚨 Breaking Changes",
        Impact::Minor => "✨ New Features",
        Impact::Patch => "🛠️ Bug Fixes",
    };

    let mut result = format!("# Version {}\n\n## {}\n", version, title);

    for file in files {
        result.push_str(&format!("- `{}` modifié\n", file));
    }

    result
}

pub fn bump_from_snapshots() -> SemVer {
    let impact = get_latest_impact().unwrap_or(Impact::Patch);
    let mut ver = load_current_version().unwrap_or(SemVer { major: 0, minor: 1, patch: 0 });

    // Récupérer le dernier snapshot et le traiter
    if let Some(snapshot) = get_latest_snapshot() {
        process_snapshot(&snapshot);
        generate_changelog(&ver, &snapshot);
    }

    ver.bump(&impact);
    write_semver(&ver);
    update_cargo_toml(&ver);

    ver
}

fn load_current_version() -> Option<SemVer> {
    std::fs::read_to_string("semver.ron").ok().and_then(|s| ron::from_str(&s).ok())
}

fn write_semver(ver: &SemVer) {
    let ron_data = ron::to_string(ver).unwrap();
    let _ = write("semver.ron", ron_data);
}

fn update_cargo_toml(ver: &SemVer) {
    let cargo = std::fs::read_to_string("Cargo.toml").unwrap();
    let mut doc = cargo.parse::<Document>().unwrap();
    doc["package"]["version"] = value(ver.to_string());
    let _ = write("Cargo.toml", doc.to_string());
}

pub fn generate_changelog(version: &SemVer, snapshot: &Snapshot) {
    let changelog_dir = ".graphver/changelog";
    let changelog_path = format!("{}/v{}.{}.{}.md", changelog_dir, version.major, version.minor, version.patch);

    create_dir_all(changelog_dir).unwrap();

    let changelog_content = format!(
        "# Changelog v{}.{}.{}\n\n- Date: {}\n- Impact: {:?}\n- Hash: {}\n\n## Modifications\n- Fichiers modifiés : (à implémenter)",
        version.major, version.minor, version.patch, chrono::Utc::now(), snapshot.impact, snapshot.hash // Utilisation du champ `hash`
    );

    std::fs::write(changelog_path, changelog_content).unwrap();
}

fn process_snapshot(snapshot: &Snapshot) {
    println!(
        "Impact: {:?}, Hash: {}",
        snapshot.impact, // Champ public
        snapshot.hash    // Champ public
    );
}

fn get_latest_snapshot() -> Option<Snapshot> {
    let path = std::path::Path::new(".graphver/snapshots");
    if !path.exists() {
        return None;
    }

    let mut entries: Vec<_> = std::fs::read_dir(path).ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("ron"))
        .collect();
    
    entries.sort_by_key(|e| {
        e.metadata()
            .ok()
            .and_then(|m| m.modified().ok())
            .map(std::cmp::Reverse)
            .unwrap_or(std::cmp::Reverse(std::time::SystemTime::UNIX_EPOCH))
    });
    
    if let Some(latest) = entries.first() {
        let content = std::fs::read_to_string(latest.path()).ok()?;
        ron::from_str(&content).ok()
    } else {
        None
    }
}
