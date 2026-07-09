use crate::analyzer::analyze_snapshot;
use crate::classifier::{classify_impact, Impact};
use crate::graph_memory::{AiEvent, AiGraph};
use crate::actions::{bump_version, revert_to};
use version_watcher::version::VersionSnapshot; // Mise à jour de l'import
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
pub enum IaStrategy {
    Conservative,
    Opportunistic,
    Safe,
}

pub fn decide_and_act(snapshot: &VersionSnapshot, graph: &mut AiGraph) {
    let analysis = analyze_snapshot(&snapshot.files_changed);
    
    if analysis.impact == Impact::Major {
        revert_to(&snapshot.id); // Utiliser revert_to pour les changements majeurs
    }

    let impact = classify_impact(&snapshot.files_changed);

    if impact == Impact::Major {
        println!("⚠️ Detected MAJOR change → bump + log");
        bump_version();
    } else if impact == Impact::Minor {
        println!("✨ Detected MINOR change → bump + log");
        bump_version();
    } else {
        println!("🛠️ Detected PATCH change → bump + log");
        bump_version();
    }

    // Enregistrer l'événement dans le graphe mémoire
    let event = AiEvent {
        snapshot_id: snapshot.id.clone(),
        impact: format!("{:?}", impact),
        changelog: format!("Changelog for version {}", snapshot.id),
        decision: format!("Bump to {:?}", impact),
        human_validated: false,
    };
    graph.add_event(event);

    // Enregistrer un feedback humain simulé pour apprentissage supervisé
    let feedback = true; // Exemple : feedback humain positif
    graph.record_human_feedback(&snapshot.id, feedback);
}

pub fn run_agent(snapshot: &VersionSnapshot, graph: &mut AiGraph, stop_signal: Arc<AtomicBool>) {
    while !stop_signal.load(Ordering::Relaxed) {
        decide_and_act(snapshot, graph);
        // Simuler une pause ou une boucle d'attente
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("🛑 IA arrêtée proprement.");
}

pub fn generate_session_report(decisions: &[String], anomalies: &[String]) {
    let mut file = File::create(".graphver/ai_activity.md").expect("Impossible de créer le fichier de rapport");
    writeln!(file, "# Journal IA – {}", chrono::Local::now().format("%d %B %Y")).unwrap();
    writeln!(file, "\n## Décisions prises").unwrap();
    for decision in decisions {
        writeln!(file, "- [✓] {}", decision).unwrap();
    }
    writeln!(file, "\n## Anomalies détectées").unwrap();
    for anomaly in anomalies {
        writeln!(file, "- {}", anomaly).unwrap();
    }
}

pub fn run_snapshot(path: &str) -> Option<String> {
    Some(format!("IA : bump suggestion pour {path}"))
}
