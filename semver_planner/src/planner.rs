use crate::loader::Snapshot;
use crate::loader::get_latest_impact;
use crate::version::SemVer;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, write};
use toml_edit::{Document, value};

#[derive(Debug, Clone, Serialize, Deserialize, bincode_next::Encode, bincode_next::Decode)]
pub enum Impact {
    Patch,
    Minor,
    Major,
}

pub fn bump_from_snapshots() -> SemVer {
    let impact = get_latest_impact().unwrap_or(Impact::Patch);
    let mut ver = load_current_version().unwrap_or(SemVer {
        major: 0,
        minor: 1,
        patch: 0,
    });
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
    std::fs::read_to_string("semver.ron")
        .ok()
        .and_then(|s| ron::from_str(&s).ok())
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
    let changelog_path = format!(
        "{}/v{}.{}.{}.md",
        changelog_dir, version.major, version.minor, version.patch
    );
    create_dir_all(changelog_dir).unwrap();
    let changelog_content = format!(
        "# Changelog v{}.{}.{}\n\n- Date: {}\n- Impact: {:?}\n- Hash: {}\n\n## Modifications\n- Fichiers modifiés : (à implémenter)",
        version.major,
        version.minor,
        version.patch,
        chrono::Utc::now(),
        snapshot.impact,
        snapshot.hash
    );
    std::fs::write(changelog_path, changelog_content).unwrap();
}

fn process_snapshot(snapshot: &Snapshot) {
    println!("Impact: {:?}, Hash: {}", snapshot.impact, snapshot.hash);
}

fn get_latest_snapshot() -> Option<Snapshot> {
    let path = std::path::Path::new(".graphver/snapshots");
    if !path.exists() {
        return None;
    }
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .ok()?
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
