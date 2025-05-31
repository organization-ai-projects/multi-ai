use chrono::Local;
use uuid::Uuid;
use super::{GraphMemory, Node, Edge};

// Structure pour stocker le résultat du parsing CLI
#[derive(Default)]
struct ParsedCliOutput {
    domains: Vec<String>,
    system_files: Vec<String>,
    status: Option<(String, bool, String, String)>, // (status, success, error_code, message)
    actions: Vec<String>,
    errors: Vec<String>,
}

// 1. PARSING DU CLI
fn parse_cli_output(output: &str) -> ParsedCliOutput {
    let mut parsed = ParsedCliOutput::default();

    // Parse les domaines
    if let Some(section) = output.split("1️⃣ Domaines détectés :").nth(1) {
        parsed.domains = section.lines()
            .skip(1)
            .take_while(|l| !l.trim().is_empty())
            .filter(|l| l.starts_with("• "))
            .map(|l| l.trim_start_matches("• ").trim().to_string())
            .collect();
    }

    // Parse les fichiers système
    if let Some(section) = output.split("2️⃣ Fichiers système :").nth(1) {
        parsed.system_files = section.lines()
            .skip(1)
            .take_while(|l| !l.trim().is_empty())
            .filter(|l| l.starts_with("• "))
            .map(|l| l.trim_start_matches("• ").trim().to_string())
            .collect();
    }

    // Parse le statut
    parsed.status = Some(if output.contains("✓ Tous les fichiers sont à jour") {
        ("ok".into(), true, "".into(), "".into())
    } else if let Some(err) = output.lines().find(|l| l.contains("Erreur") || l.contains("❌")) {
        ("error".into(), false, err.trim().into(), err.trim().into())
    } else if output.contains("⚠️") {
        ("warning".into(), false, "warning".into(), "".into())
    } else {
        ("unknown".into(), false, "".into(), "".into())
    });

    // Parse les actions et erreurs
    for line in output.lines() {
        if line.contains("sera mis à jour") {
            parsed.actions.push(line.trim().to_string());
        }
        if line.contains("Erreur") || line.contains("❌") {
            parsed.errors.push(line.trim().to_string());
        }
    }

    parsed
}

// 2. MISE À JOUR DE LA MÉMOIRE
pub fn parse_cli_output_and_update_graph(mem: &mut GraphMemory, output: &str, run_id: &str, parent_run: Option<&str>) {
    let now = Local::now();
    let parsed = parse_cli_output(output);
    
    // Créer le nœud Run parent
    let run_node_id = create_run_node(mem, run_id, parent_run, &now);

    // Ajouter les domaines
    for domain in parsed.domains {
        add_domain_node(mem, &domain, &run_node_id, &now);
    }

    // Ajouter les fichiers système
    for sys_file in parsed.system_files {
        add_system_file_node(mem, &sys_file, &run_node_id, &now);
    }

    // Ajouter le statut
    if let Some((status, success, code, msg)) = parsed.status {
        add_status_node(mem, &status, success, &code, &msg, &run_node_id, &now);
    }

    // Ajouter les actions et erreurs
    for action in parsed.actions {
        add_action_node(mem, &action, &run_node_id, &now);
    }
    for error in parsed.errors {
        add_error_node(mem, &error, &run_node_id, &now);
    }

    // Gérer le lien parent si présent
    if let Some(parent) = parent_run {
        mem.add_edge(Edge {
            from: run_node_id,
            to: parent.to_string(),
            label: "parent_run".to_string(),
        });
    }
}

fn create_run_node(mem: &mut GraphMemory, run_id: &str, parent_run: Option<&str>, now: &chrono::DateTime<Local>) -> String {
    let run_node_id = Uuid::now_v7().to_string();
    let run_type = if run_id.contains("dry") { "dry-run" } else { "apply" };
    let tags = vec![run_type];

    let mut props = vec![
        ("timestamp".into(), now.to_rfc3339()),
        ("run_type".into(), run_type.into()),
        ("tags".into(), tags.join(","))
    ];
    
    if let Some(parent) = parent_run {
        props.push(("parent_run".into(), parent.to_string()));
    }

    mem.add_or_update_node(Node {
        id: run_node_id.clone(),
        label: "Run".to_string(),
        properties: props.into_iter().collect(),
    });

    run_node_id
}

fn add_domain_node(mem: &mut GraphMemory, domain_name: &str, run_node_id: &str, now: &chrono::DateTime<Local>) {
    let domain_id = Uuid::now_v7().to_string();
    mem.add_or_update_node(Node {
        id: domain_id.clone(),
        label: "Domain".to_string(),
        properties: [
            ("name".into(), domain_name.to_string()),
            ("last_seen".into(), now.to_rfc3339()),
        ].iter().cloned().collect(),
    });
    mem.add_edge(Edge {
        from: run_node_id.clone(),
        to: domain_id.clone(),
        label: "detected_domain".to_string(),
    });
}

fn add_system_file_node(mem: &mut GraphMemory, sys_name: &str, run_node_id: &str, now: &chrono::DateTime<Local>) {
    let sys_id = Uuid::now_v7().to_string();
    mem.add_or_update_node(Node {
        id: sys_id.clone(),
        label: "SystemFile".to_string(),
        properties: [
            ("name".into(), sys_name.to_string()),
            ("last_seen".into(), now.to_rfc3339()),
        ].iter().cloned().collect(),
    });
    mem.add_edge(Edge {
        from: run_node_id.clone(),
        to: sys_id.clone(),
        label: "detected_systemfile".to_string(),
    });
}

fn add_status_node(mem: &mut GraphMemory, status: &str, success: bool, error_code: &str, error_message: &str, run_node_id: &str, now: &chrono::DateTime<Local>) {
    let status_id = Uuid::now_v7().to_string();
    let mut status_props = vec![
        ("status".into(), status.into()),
        ("timestamp".into(), now.to_rfc3339()),
        ("success".into(), success.to_string()),
    ];
    if !error_code.is_empty() {
        status_props.push(("error_code".into(), error_code.to_string()));
    }
    if !error_message.is_empty() {
        status_props.push(("error_message".into(), error_message.to_string()));
    }
    mem.add_or_update_node(Node {
        id: status_id.clone(),
        label: "RunStatus".to_string(),
        properties: status_props.into_iter().collect(),
    });
    mem.add_edge(Edge {
        from: run_node_id.clone(),
        to: status_id.clone(),
        label: "status".to_string(),
    });
}

fn add_action_node(mem: &mut GraphMemory, action: &str, run_node_id: &str, now: &chrono::DateTime<Local>) {
    let action_id = Uuid::now_v7().to_string();
    mem.add_or_update_node(Node {
        id: action_id.clone(),
        label: "Action".to_string(),
        properties: [
            ("action".into(), action.into()),
            ("target".into(), "mod.rs or trait_commands.rs".into()),
            ("timestamp".into(), now.to_rfc3339()),
        ].iter().cloned().collect(),
    });
    mem.add_edge(Edge {
        from: run_node_id.clone(),
        to: action_id,
        label: "suggested_action".to_string(),
    });
}

fn add_error_node(mem: &mut GraphMemory, message: &str, run_node_id: &str, now: &chrono::DateTime<Local>) {
    let err_id = Uuid::now_v7().to_string();
    mem.add_or_update_node(Node {
        id: err_id.clone(),
        label: "Error".to_string(),
        properties: [
            ("message".into(), message.trim().to_string()),
            ("timestamp".into(), now.to_rfc3339()),
        ].iter().cloned().collect(),
    });
    mem.add_edge(Edge {
        from: run_node_id.clone(),
        to: err_id,
        label: "error".to_string(),
    });
}

fn link_domains_to_status(mem: &mut GraphMemory, status_id: &str) {
    let domain_ids: Vec<_> = mem.nodes.iter()
        .filter(|n| n.label == "Domain")
        .map(|n| n.id.clone())
        .collect();
        
    for domain_id in domain_ids {
        mem.add_edge(Edge {
            from: domain_id,
            to: status_id.to_string(),
            label: "has_status".to_string(),
        });
    }
}
