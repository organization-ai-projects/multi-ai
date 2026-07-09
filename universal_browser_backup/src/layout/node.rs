//! # Nœuds de l'arbre de layout
//! 
//! Ce fichier est responsable de:
//! - Définir la structure des nœuds de layout (LayoutNode)
//! - Gérer les propriétés visuelles des nœuds (couleur, bordure)
//! - Calculer la disposition des enfants selon différentes stratégies
//! - Fournir des méthodes de recherche dans l'arbre
//! 
//! Ce fichier NE DOIT PAS contenir:
//! - Du code de rendu direct sur les pixels
//! - Des configurations spécifiques à une vue
//! - Des logiques de gestion d'événements
//! 
//! Il définit la structure arborescente du layout et les algorithmes de disposition.

use crate::layout::Rect;
use crate::renderer::Color;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutDirection {
    Horizontal,
    Vertical,
    Grid(u32, u32), // colonnes, lignes
}

#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub rect: Rect,
    pub direction: LayoutDirection,
    pub children: Vec<LayoutNode>,
    pub background_color: Color,
    pub border_color: Option<Color>,
    pub id: String,
    pub focusable: bool,
    pub visible: bool,
}

impl LayoutNode {
    pub fn new(id: &str, rect: Rect, direction: LayoutDirection) -> Self {
        Self {
            rect,
            direction,
            children: Vec::new(),
            background_color: Color::transparent(),
            border_color: None,
            id: id.to_string(),
            focusable: false,
            visible: true,
        }
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background_color = color;
        self
    }

    pub fn with_border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    pub fn add_child(&mut self, child: LayoutNode) {
        self.children.push(child);
    }

    pub fn set_focusable(&mut self, focusable: bool) {
        self.focusable = focusable;
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn calculate_layout(&mut self) {
        if self.children.is_empty() {
            return;
        }

        match self.direction {
            LayoutDirection::Horizontal => self.calculate_horizontal_layout(),
            LayoutDirection::Vertical => self.calculate_vertical_layout(),
            LayoutDirection::Grid(cols, rows) => self.calculate_grid_layout(cols, rows),
        }

        // Récursivement calculer le layout des enfants
        for child in &mut self.children {
            child.calculate_layout();
        }
    }

    fn calculate_horizontal_layout(&mut self) {
        let child_count = self.children.len() as u32;
        if child_count == 0 {
            return;
        }

        let child_width = self.rect.width / child_count;
        for (i, child) in self.children.iter_mut().enumerate() {
            child.rect = Rect::new(
                self.rect.x + (i as u32 * child_width),
                self.rect.y,
                child_width,
                self.rect.height,
            );
        }
    }

    fn calculate_vertical_layout(&mut self) {
        let child_count = self.children.len() as u32;
        if child_count == 0 {
            return;
        }

        let child_height = self.rect.height / child_count;
        for (i, child) in self.children.iter_mut().enumerate() {
            child.rect = Rect::new(
                self.rect.x,
                self.rect.y + (i as u32 * child_height),
                self.rect.width,
                child_height,
            );
        }
    }

    fn calculate_grid_layout(&mut self, cols: u32, rows: u32) {
        if cols == 0 || rows == 0 {
            return;
        }

        let cell_width = self.rect.width / cols;
        let cell_height = self.rect.height / rows;

        for (i, child) in self.children.iter_mut().enumerate() {
            let i = i as u32;
            let col = i % cols;
            let row = i / cols;

            if row < rows {
                child.rect = Rect::new(
                    self.rect.x + (col * cell_width),
                    self.rect.y + (row * cell_height),
                    cell_width,
                    cell_height,
                );
            }
        }
    }

    // Suppression de la méthode render non utilisée
    // pub fn render(&self, renderer: &crate::renderer::Renderer, frame: &mut [u8]) {
    //    // ...
    // }

    pub fn find_node_at(&self, x: u32, y: u32) -> Option<&LayoutNode> {
        if !self.rect.contains(x, y) || !self.visible {
            return None;
        }

        // Chercher d'abord dans les enfants (premier enfant trouvé)
        for child in &self.children {
            if let Some(node) = child.find_node_at(x, y) {
                return Some(node);
            }
        }

        // Si aucun enfant ne contient le point, retourner self
        Some(self)
    }

    pub fn find_node_by_id(&self, id: &str) -> Option<&LayoutNode> {
        if self.id == id {
            return Some(self);
        }

        for child in &self.children {
            if let Some(node) = child.find_node_by_id(id) {
                return Some(node);
            }
        }

        None
    }

    // Ajouter la méthode pour trouver un nœud mutable
    pub fn find_node_by_id_mut(&mut self, id: &str) -> Option<&mut LayoutNode> {
        if self.id == id {
            return Some(self);
        }

        for child in &mut self.children {
            if let Some(node) = child.find_node_by_id_mut(id) {
                return Some(node);
            }
        }

        None
    }
}
