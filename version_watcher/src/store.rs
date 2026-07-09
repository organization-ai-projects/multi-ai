use crate::version::VersionSnapshot;
use crate::graph::{Impact, GraphNode, VersionGraph, load_graph};
use std::fs::{create_dir_all, write};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use walkdir::WalkDir;
use chrono::Utc;
use std::collections::HashSet;
use std::fs;
use std::io;

pub fn create_snapshot(path: &str) {
    let hash = hash_folder(path);
    let timestamp = Utc::now().timestamp();
    let id = Uuid::new_v4().to_string();

    // Utiliser estimate_impact au lieu de calculate_impact
    let impact = estimate_impact(&hash);

    let files_changed = list_files(path);

    let snapshot = VersionSnapshot {
        id: id.clone(),
        timestamp,
        path: path.to_string(),
        impact: impact.clone(),
        hash: hash.clone(),
        files_changed,
    };

    // Sauvegarder le snapshot
    let _ = create_dir_all(".graphver/snapshots");

    // Sauvegarder en JSON pour la compatibilité universelle
    let json_path = format!(".graphver/snapshots/{}.json", id);
    write(&json_path, serde_json::to_string_pretty(&snapshot).unwrap()).unwrap();

    // Sauvegarder en binaire pour la performance
    let bin_path = format!(".graphver/snapshots/{}.bin", id);
    write(&bin_path, &bincode::serialize(&snapshot).unwrap()).unwrap();

    let graph_node = GraphNode {
        id: id.clone(),
        timestamp,
        hash,
        impact,
    };

    update_graph(graph_node);
    println!("✅ Snapshot '{}' saved", id);
}

// Fonction pour lister les fichiers dans un dossier
fn list_files(path: &str) -> Vec<String> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_string_lossy().to_string())
        .collect()
}

fn update_graph(node: GraphNode) {
    let mut graph = load_graph(); // Utilisation de load_graph au lieu de la logique custom
    graph.add_node(node);
    write(".graphver/graph.ron", ron::to_string(&graph).unwrap()).unwrap();
}

fn hash_folder(path: &str) -> String {
    let mut hasher = Sha256::new();
    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            if let Ok(content) = std::fs::read(entry.path()) {
                hasher.update(&content);
            }
        }
    }
    format!("{:x}", hasher.finalize())
}

fn estimate_impact(hash: &str) -> Impact {
    let previous_hash = get_previous_hash();
    estimate_impact_from_diff(&previous_hash, hash)
}

fn get_previous_hash() -> String {
    let graph_path = ".graphver/graph.ron";
    if let Ok(graph_data) = std::fs::read_to_string(graph_path) {
        if let Ok(graph) = ron::from_str::<VersionGraph>(&graph_data) {
            if let Some(latest_id) = graph.latest_id() {
                if let Some(node) = graph.nodes.get(&latest_id) {
                    return node.hash.clone();
                }
            }
        }
    }
    String::new() // Retourne une chaîne vide si aucun hash précédent n'est trouvé
}

fn estimate_impact_from_diff(old_hash: &str, new_hash: &str) -> Impact {
    if old_hash == new_hash {
        Impact::Patch
    } else {
        let diff_count = count_diff_files(old_hash, new_hash);
        if diff_count < 3 {
            Impact::Minor
        } else {
            Impact::Major
        }
    }
}

fn count_diff_files(old_hash: &str, new_hash: &str) -> usize {
    let old_files = get_files_from_hash(old_hash);
    let new_files = get_files_from_hash(new_hash);

    let old_set: HashSet<_> = old_files.into_iter().collect();
    let new_set: HashSet<_> = new_files.into_iter().collect();

    let added_files = new_set.difference(&old_set).count();
    let removed_files = old_set.difference(&new_set).count();

    added_files + removed_files
}

fn get_files_from_hash(hash: &str) -> Vec<String> {
    // Essayer d'abord le format binaire pour la performance
    let bin_path = format!(".graphver/snapshots/{}.bin", hash);
    if let Ok(snapshot_data) = std::fs::read(&bin_path) {
        if let Ok(snapshot) = bincode::deserialize::<VersionSnapshot>(&snapshot_data) {
            return snapshot.files_changed;
        }
    }

    // Fallback sur JSON si le binaire n'est pas disponible
    let json_path = format!(".graphver/snapshots/{}.json", hash);
    if let Ok(snapshot_data) = std::fs::read_to_string(&json_path) {
        if let Ok(snapshot) = serde_json::from_str::<VersionSnapshot>(&snapshot_data) {
            return snapshot.files_changed;
        }
    }

    Vec::new() // Retourne une liste vide si le snapshot est introuvable
}

pub fn revert_snapshot(id: &str) {
    // Essayer d'abord le format binaire
    let bin_path = format!(".graphver/snapshots/{}.bin", id);
    let json_path = format!(".graphver/snapshots/{}.json", id);

    let snapshot = if let Ok(data) = std::fs::read(&bin_path) {
        bincode::deserialize::<VersionSnapshot>(&data).ok()
    } else if let Ok(data) = std::fs::read_to_string(&json_path) {
        serde_json::from_str::<VersionSnapshot>(&data).ok()
    } else {
        None
    };

    if let Some(snapshot) = snapshot {
        // Récupérer les fichiers actuels et cibles
        let current_files = get_files_from_hash(&snapshot.hash);
        let target_files = snapshot.files_changed;
        
        // Restaurer les fichiers
        for file in &target_files {
            if !current_files.contains(file) {
                // Le fichier n'existe plus, on doit le restaurer
                if let Ok(content) = std::fs::read_to_string(&format!(".graphver/files/{}/{}", snapshot.hash, file)) {
                    let _ = std::fs::write(file, content);
                    println!("✅ Restauré : {}", file);
                }
            }
        }
        
        // Supprimer les fichiers qui n'existaient pas dans le snapshot
        for file in &current_files {
            if !target_files.contains(file) {
                let _ = fs::remove_file(file);
                println!("🗑️ Supprimé : {}", file);
            }
        }

        let old_hash = get_previous_hash();
        let diff_count = count_diff_files(&old_hash, &snapshot.hash);
        
        println!("📊 Analyse du revert :");
        println!("- Fichiers modifiés : {}", diff_count);
        println!("- Impact estimé : {:?}", estimate_impact_from_diff(&old_hash, &snapshot.hash));

        let graph_node = GraphNode {
            id: format!("revert-{}", id),
            timestamp: Utc::now().timestamp(),
            hash: snapshot.hash.clone(),
            impact: Impact::Patch,
        };
        update_graph(graph_node);
        
        println!("✅ Revert effectué vers le snapshot '{}'", id);
    } else {
        eprintln!("❌ Impossible de trouver le snapshot '{}'", id);
    }
}

pub fn delete_snapshot(id: &str) -> Result<(), io::Error> {
    let snapshot_path = format!(".graphver/snapshots/{}.json", id);
    if fs::metadata(&snapshot_path).is_ok() {
        fs::remove_file(snapshot_path)?;
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Snapshot introuvable"))
    }
}
