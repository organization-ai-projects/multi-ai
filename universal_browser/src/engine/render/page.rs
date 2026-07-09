// Rendu de pages complètes

use super::primitives::BACKGROUND_COLOR;
use super::ui::{draw_button, draw_text_block, draw_url_bar, UiState};
use crate::engine::runtime::Runtime;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use unilang::ast::Expr;

// Variable statique sécurisée pour stocker la dernière page
static LAST_PAGE: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));

/// Rendu principal d'une page complète
pub fn render_page(runtime: &Runtime, frame: &mut [u8], width: u32, ui_state: &mut UiState) {
    // Effacer l'écran avec la couleur de fond
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&BACKGROUND_COLOR);
    }

    // Dessiner la barre d'URL
    let url_bar_height = draw_url_bar(frame, width, ui_state);

    // Rendu de la page actuelle
    let page_name = runtime.current_page_name();

    // Vérifier si la page a changé
    let mut last_page = LAST_PAGE.lock().unwrap();
    if last_page.as_ref() != Some(&page_name) {
        println!("Rendu de la page: {}", page_name);
        *last_page = Some(page_name.clone());
    }
    drop(last_page);

    // Disposition verticale des composants après la barre d'URL
    let mut y_offset = url_bar_height + 20;

    // Pour éviter des messages de débogage en double
    let mut current_components = std::collections::HashSet::new();

    // Rendu des composants de la page
    for comp in runtime.current_components() {
        // Créer un identifiant unique pour ce composant
        let comp_id = format!("{}-{:?}", comp.name, comp.properties);
        current_components.insert(comp_id.clone());

        match comp.name.as_str() {
            "Text" => {
                if let Some((_, Expr::String(msg))) =
                    comp.properties.iter().find(|(k, _)| k == "content")
                {
                    // Centrer le texte horizontalement
                    let x_pos = (width - msg.len() as u32 * 10) / 2;

                    // Dessiner le bloc de texte
                    let (_, _, new_y) = draw_text_block(msg, x_pos, y_offset, frame, width);

                    if !ui_state.rendered_components.contains(&comp_id) {
                        println!("Rendu Text: {} à y={}", msg, y_offset);
                    }

                    y_offset = new_y + 10; // Espace entre composants
                }
            }
            "Button" => {
                if let Some((_, Expr::String(label))) =
                    comp.properties.iter().find(|(k, _)| k == "content")
                {
                    // Centrer le bouton horizontalement
                    let button_width = (label.len() as u32 * 10).max(150);
                    let x_pos = (width - button_width) / 2;

                    // Dessiner le bouton
                    let (_, _, new_y) = draw_button(label, x_pos, y_offset, frame, width);

                    if !ui_state.rendered_components.contains(&comp_id) {
                        println!("Rendu Button: {} à y={}", label, y_offset);
                    }

                    y_offset = new_y + 20; // Plus d'espace après les boutons
                }
            }
            _ => {
                if !ui_state.rendered_components.contains(&comp_id) {
                    println!("Composant non pris en charge: {}", comp.name);
                }
            }
        }
    }

    // Mettre à jour l'état
    ui_state.rendered_components = current_components;
}
