//! # Gestionnaire global du layout
//! 
//! Ce fichier est responsable de:
//! - Gérer l'arbre complet de layout
//! - Suivre l'élément actuellement en focus
//! - Coordonner le calcul du layout
//! - Gérer les interactions de base avec les éléments (clic, focus)
//! 
//! Ce fichier NE DOIT PAS contenir:
//! - Des définitions spécifiques de vues ou composants
//! - Du code de rendu direct
//! - Des logiques d'application métier
//! 
//! Il orchestre les nœuds de layout et maintient l'état global du layout.

use crate::layout::{LayoutDirection, LayoutNode, Rect};

// Gestionnaire de layout global
pub struct LayoutManager {
    pub root: LayoutNode,
    pub focused_id: Option<String>,
}

impl LayoutManager {
    pub fn new(width: u32, height: u32) -> Self {
        let root = LayoutNode::new(
            "root",
            Rect::new(0, 0, width, height),
            LayoutDirection::Vertical,
        );

        Self {
            root,
            focused_id: None,
        }
    }

    pub fn calculate_layout(&mut self) {
        self.root.calculate_layout();
    }

    // Cette méthode est utilisée lors du clic sur un élément
    pub fn handle_click(&mut self, x: u32, y: u32) -> bool {
        if let Some(node) = self.root.find_node_at(x, y) {
            if node.focusable {
                self.focused_id = Some(node.id.clone());

                // Mettre à jour la vue en fonction de l'élément cliqué
                match node.id.as_str() {
                    "btn_home" | "option_welcome" => return true,
                    "btn_explorer" | "option_ia" => return true,
                    "btn_graph" | "option_graph" => return true,
                    "btn_files" | "option_files" => return true,
                    _ => {}
                }

                return true;
            }
        }
        false
    }

    pub fn focus_next(&mut self) -> bool {
        // Simple implémentation: trouver tous les nœuds focusables et passer au suivant
        let mut focusable_nodes = Vec::new();
        self.collect_focusable_nodes(&self.root, &mut focusable_nodes);

        if focusable_nodes.is_empty() {
            return false;
        }

        // Trouver l'index du nœud actuellement focus
        let current_index = match &self.focused_id {
            Some(id) => focusable_nodes
                .iter()
                .position(|n| n.id == *id)
                .unwrap_or(0),
            None => 0,
        };

        // Passer au suivant (boucler si nécessaire)
        let next_index = (current_index + 1) % focusable_nodes.len();
        self.focused_id = Some(focusable_nodes[next_index].id.clone());

        true
    }

    fn collect_focusable_nodes<'a>(&self, node: &'a LayoutNode, nodes: &mut Vec<&'a LayoutNode>) {
        if node.focusable && node.visible {
            nodes.push(node);
        }

        for child in &node.children {
            self.collect_focusable_nodes(child, nodes);
        }
    }
}
