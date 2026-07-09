//! # Implémentation principale du renderer
//!
//! Ce fichier est responsable de:
//! - Initialiser et gérer le système de rendu (pixels)
//! - Coordonner le cycle de rendu global
//! - Gérer le redimensionnement du rendu
//!
//! Ce fichier NE DOIT PAS contenir:
//! - Des détails de rendu spécifiques aux vues
//! - Des algorithmes complexes de dessin
//! - Du code de rendu du layout
//!
//! Il s'occupe uniquement de la coordination du processus de rendu.

use super::{
    color::Color,
    layout_renderer::render_layout,
    primitives::draw_rect,
    view_renderers::render_current_view,
};
use crate::state::AppState;
use pixels::{Pixels, SurfaceTexture};
use winit::window::Window;

pub struct Renderer {
    pixels: Pixels,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, String> {
        let size = window.inner_size();
        let width = size.width;
        let height = size.height;
        let surface_texture = SurfaceTexture::new(width, height, window);
        let pixels = Pixels::new(width, height, surface_texture).map_err(|e| e.to_string())?;
        Ok(Self {
            pixels,
            width,
            height,
        })
    }

    pub fn render(&mut self, state: &AppState) -> Result<(), String> {
        // Obtenir un accès au frame et remplir avec du noir
        {
            let frame = self.pixels.frame_mut();
            frame.fill(0);
        }

        // Stocker localement les dimensions pour éviter les emprunts multiples
        let width = self.width;
        let height = self.height;

        // Effectuer le rendu dans le frame
        {
            let frame = self.pixels.frame_mut();

            // Dessiner l'arrière-plan de base - au lieu d'appeler self.draw_background
            // qui crée un double emprunt, on dessine directement ici
            draw_rect(
                0,
                0,
                width,
                40,
                Color::new(100, 149, 237, 255),
                frame,
                width,
                height,
            );

            // Barre latérale en gris
            draw_rect(
                0,
                40,
                200,
                height - 40,
                Color::new(220, 220, 220, 255),
                frame,
                width,
                height,
            );

            // Zone principale en blanc
            draw_rect(
                200,
                40,
                width - 200,
                height - 40,
                Color::new(255, 255, 255, 255),
                frame,
                width,
                height,
            );

            // Rendu de la vue actuelle (délégué au module dédié)
            render_current_view(state, frame, width, height);

            // Rendu du layout (délégué au module dédié)
            render_layout(&state.layout_manager, frame, width, height);
        }

        self.pixels.render().map_err(|e| e.to_string())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        // Mettre à jour les dimensions stockées
        self.width = width;
        self.height = height;

        if let Err(e) = self.pixels.resize_surface(width, height) {
            log::error!("Erreur lors du redimensionnement de la surface: {}", e);
        }
        if let Err(e) = self.pixels.resize_buffer(width, height) {
            log::error!("Erreur lors du redimensionnement du buffer: {}", e);
        }
    }
}
