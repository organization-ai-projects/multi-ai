use std::collections::HashMap;
use crate::models::ChangeNode;

pub struct ChangeGraph {
    nodes: Vec<ChangeNode>,
    edges: Vec<(usize, usize)>,
}

impl ChangeGraph {
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph changes {\n");
        
        // Nodes
        for (i, node) in self.nodes.iter().enumerate() {
            let color = match node.impact.as_str() {
                "major" => "red",
                "minor" => "yellow",
                "patch" => "green",
                _ => "gray"
            };
            dot.push_str(&format!("    n{} [label=\"{}\", color={}];\n", 
                i, node.path, color));
        }

        // Edges
        for (from, to) in &self.edges {
            dot.push_str(&format!("    n{} -> n{};\n", from, to));
        }

        dot.push_str("}\n");
        dot
    }

    pub fn to_html(&self) -> String {
        let mut html = String::from(r#"
        <!DOCTYPE html>
        <html>
        <head>
            <script src="https://d3js.org/d3.v7.min.js"></script>
            <style>
                .node { fill: #ccc; }
                .major { fill: #ff6b6b; }
                .minor { fill: #ffe66d; }
                .patch { fill: #4ecdc4; }
            </style>
        </head>
        <body>
            <svg id="graph"></svg>
            <script>
        "#);

        // Injecter les données
        html.push_str(&format!("const data = {};", 
            serde_json::to_string(&self.nodes).unwrap()));

        // Ajouter le code D3.js pour la visualisation
        html.push_str(r#"
            // D3.js visualization code here
            </script>
        </body>
        </html>
        "#);

        html
    }
}
