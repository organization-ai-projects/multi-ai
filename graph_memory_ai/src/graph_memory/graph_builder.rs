use chrono::Local;
use uuid::Uuid;
use super::graph_memory::{GraphMemory, Node, Edge}; // Correction ici
use super::cli_parser::CliOutput;

pub struct GraphBuilder<'a> {
    mem: &'a mut GraphMemory,
}

impl<'a> GraphBuilder<'a> {
    pub fn new(mem: &'a mut GraphMemory) -> Self {
        Self { mem }
    }

    pub fn build_cli_run(&mut self, output: CliOutput, run_id: &str, parent_run: Option<&str>) -> String {
        let now = Local::now();
        let run_node_id = Uuid::now_v7().to_string();

        // Ajouter le nœud run
        self.add_node(Node {
            id: run_node_id.clone(),
            label: "Run".to_string(),
            properties: self.create_run_properties(run_id, parent_run, &now),
        });

        // Ajouter les éléments détectés
        self.add_run_elements(&run_node_id, output, &now);

        // Lier au parent si présent
        if let Some(parent) = parent_run {
            self.add_edge(Edge {
                from: run_node_id.clone(),
                to: parent.to_string(),
                label: "parent_run".to_string(),
            });
        }

        run_node_id
    }

    fn add_node(&mut self, node: Node) {
        self.mem.add_or_update_node(node);
    }

    fn add_edge(&mut self, edge: Edge) {
        self.mem.add_edge(edge);
    }

    fn create_run_properties(&self, run_id: &str, parent_run: Option<&str>, now: &chrono::DateTime<Local>) -> std::collections::HashMap<String, String> {
        let mut props = std::collections::HashMap::new();
        props.insert("timestamp".into(), now.to_rfc3339());
        props.insert("run_type".into(), if run_id.contains("dry") { "dry-run" } else { "apply" }.into());
        if let Some(parent) = parent_run {
            props.insert("parent_run".into(), parent.to_string());
        }
        props
    }

    fn add_run_elements(&mut self, run_id: &str, output: CliOutput, now: &chrono::DateTime<Local>) {
        // Ajout des domaines et fichiers système
        for domain in output.domains {
            let node_id = Uuid::now_v7().to_string();
            self.add_node_with_link(
                "Domain",
                node_id.clone(),
                vec![("name", domain)],
                run_id,
                "detected_domain",
                now
            );
        }
        for file in output.system_files {
            let node_id = Uuid::now_v7().to_string();
            self.add_node_with_link(
                "SystemFile",
                node_id.clone(),
                vec![("name", file)],
                run_id,
                "detected_systemfile",
                now
            );
        }

        // Ajout du statut s'il existe
        if let Some(status) = output.status {
            let status_id = Uuid::now_v7().to_string();
            let status_str = status.status.clone();
            let error_msg = status.error_message.clone();
            
            let mut props: std::collections::HashMap<String, String> = std::collections::HashMap::new();
            props.insert("status".into(), status_str.clone());
            props.insert("success".into(), status.success.to_string());
            props.insert("error_code".into(), status.error_code);
            props.insert("error_message".into(), error_msg.clone());
            
            self.add_node_with_link(
                "Status",
                status_id,
                vec![
                    ("status", status_str),
                    ("error", error_msg)
                ],
                run_id,
                "run_status",
                now
            );
        }
    }

    fn add_node_with_link(
        &mut self,
        label: &str,
        node_id: String,
        properties: Vec<(&str, String)>,
        run_id: &str,
        link_label: &str,
        now: &chrono::DateTime<Local>
    ) {
        let mut props: std::collections::HashMap<_, _> = properties.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        props.insert("timestamp".into(), now.to_rfc3339());

        self.add_node(Node {
            id: node_id.clone(),
            label: label.to_string(),
            properties: props,
        });

        self.add_edge(Edge {
            from: run_id.to_string(),
            to: node_id,
            label: link_label.to_string(),
        });
    }
}
