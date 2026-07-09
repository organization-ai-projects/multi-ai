//! # Gestionnaire de l'état global de l'application
//!
//! Ce fichier est responsable de:
//! - Définir et gérer l'état global de l'application
//! - Stocker la vue active actuelle
//! - Gérer les transitions entre vues
//! - Coordonner l'état de l'interface utilisateur
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Du code de rendu
//! - Des définitions détaillées de layout
//! - Des gestionnaires d'événements
//!
//! Il centralise l'état et fournit des méthodes pour le manipuler de façon contrôlée.

use crate::layout::{update_panel_visibility, views::ViewManager, LayoutManager};
use crate::navigation::Navigator;
use crate::plugins::PluginManager;
use crate::ws_bridge::WsBridge;
use winit::dpi::PhysicalSize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Welcome,
    IAExplorer,
    GraphMemory,
    FileViewer,
}

pub struct AppState {
    pub size: PhysicalSize<u32>,
    pub redraw_requested: bool,
    pub current_view: ViewMode,
    pub layout_manager: LayoutManager,
    pub view_manager: ViewManager,
    pub navigator: Navigator,
    pub plugin_manager: PluginManager,
    pub ws_bridge: WsBridge,
    pub mouse_x: u32,
    pub mouse_y: u32,
    pub is_mouse_pressed: bool,
}

impl AppState {
    pub fn new(size: PhysicalSize<u32>) -> Self {
        let layout_manager = LayoutManager::new(size.width, size.height);
        let view_manager = ViewManager::new();
        let navigator = Navigator::new();
        let plugin_manager = PluginManager::new();
        let ws_bridge = WsBridge::new();

        Self {
            size,
            redraw_requested: true,
            current_view: ViewMode::Welcome,
            layout_manager,
            view_manager,
            navigator,
            plugin_manager,
            ws_bridge,
            mouse_x: 0,
            mouse_y: 0,
            is_mouse_pressed: false,
        }
    }

    pub fn switch_view(&mut self, view_mode: ViewMode) {
        self.current_view = view_mode;

        // Utiliser la fonction globale pour mettre à jour la visibilité
        update_panel_visibility(self);

        // Mettre à jour le focus sur l'option correspondante dans la sidebar
        let option_id = self.view_manager.get_option_id_for_view(view_mode);
        if let Some(node) = self.layout_manager.root.find_node_by_id(option_id) {
            self.layout_manager.focused_id = Some(node.id.clone());
        }

        self.redraw_requested = true;
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.size = PhysicalSize::new(width, height);
        self.layout_manager = LayoutManager::new(width, height);
        self.layout_manager.calculate_layout();
        self.redraw_requested = true;
    }

    pub fn update_mouse(&mut self, x: u32, y: u32) {
        self.mouse_x = x;
        self.mouse_y = y;
        self.redraw_requested = true;
    }

    pub fn handle_mouse_press(&mut self, pressed: bool) {
        self.is_mouse_pressed = pressed;
        if pressed {
            self.layout_manager.handle_click(self.mouse_x, self.mouse_y);
        }
        self.redraw_requested = true;
    }

    pub fn focus_next_component(&mut self) {
        if self.layout_manager.focus_next() {
            self.redraw_requested = true;
        }
    }
}
