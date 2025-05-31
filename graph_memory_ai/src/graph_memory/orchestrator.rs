use super::{
    cli_parser::parse_cli_output,
    graph_builder::GraphBuilder,
    graph_memory::{GraphMemory, Node, Edge}, // Correction ici
};

pub struct Orchestrator {
    memory: GraphMemory,
    last_run_id: Option<String>,
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            memory: GraphMemory::default(),
            last_run_id: None,
        }
    }

    pub fn load(path: &str) -> Result<Self, String> {
        Ok(Self {
            memory: GraphMemory::load_ron(path)?,
            last_run_id: None,
        })
    }

    pub fn process_cli_output(&mut self, output: &str, run_id: &str) -> String {
        // 1. Parser la sortie CLI
        let cli_output = parse_cli_output(output);
        
        // 2. Construire le graphe avec le contexte du dernier run
        let mut builder = GraphBuilder::new(&mut self.memory);
        let new_run_id = builder.build_cli_run(cli_output, run_id, self.last_run_id.as_deref());
        
        // 3. Mémoriser ce run comme contexte
        self.last_run_id = Some(new_run_id.clone());
        
        new_run_id
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        self.memory.save_ron(path)
    }

    pub fn save_bin(&self, path: &str) -> Result<(), String> {
        self.memory.save_bin(path)
    }

    pub fn get_memory(&self) -> &GraphMemory {
        &self.memory
    }

    pub fn get_memory_mut(&mut self) -> &mut GraphMemory {
        &mut self.memory
    }

    // Accès aux noeuds et liens
    pub fn nodes(&self) -> &Vec<Node> {
        &self.memory.nodes
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.memory.edges
    }

    // Manipulation du graphe
    pub fn add_or_update_node(&mut self, node: Node) {
        self.memory.add_or_update_node(node);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.memory.add_edge(edge);
    }
}
