use crate::engine::input::handle_input;
use crate::engine::render::{render_page, UiState};
use crate::engine::router::navigate_to_url;
use crate::engine::runtime::Runtime;
use crate::utils::sanitize_input;
use pixels::Pixels;
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event, KeyEvent, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::Window;

pub struct Browser {
    pixels: Pixels,
    runtime: Runtime,
    ui_state: UiState,
}

impl Browser {
    pub fn new(pixels: Pixels) -> Self {
        // Page d'accueil du navigateur
        let source = r#"
            function view_Home {
                Text { content: "Universal Browser" }
                Text { content: "Navigateur pour sites .unl" }
                Button { content: "Visiter example.unl" }
                Text { content: "Cliquez sur la barre d'URL en haut pour naviguer" }
                Text { content: "Format: unl://nom_du_site" }
                Text { content: "Exemple: unl://example" }
            }
        "#;

        Self {
            pixels,
            runtime: Runtime::new(source),
            ui_state: UiState::new(),
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>, window: Window) {
        let mut runtime = self.runtime.clone();
        let mut ui_state = self.ui_state;

        // Variables pour limiter le taux de rafraîchissement
        let mut last_render = Instant::now();
        let frame_rate_limit = Duration::from_millis(33); // ~30 FPS
        let mut needs_redraw = true;

        // Variable pour stocker la position actuelle du curseur
        let mut cursor_position = (0.0, 0.0);

        // Horodatage de la dernière saisie pour éviter les répétitions
        let mut last_key_timestamp = Instant::now();
        let key_repeat_delay = Duration::from_millis(150); // Délai entre les répétitions de touches

        event_loop
            .run(move |event, elwt| {
                elwt.set_control_flow(ControlFlow::Wait);

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                            return;
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            // Mettre à jour la position du curseur lorsqu'il se déplace
                            cursor_position = (position.x, position.y);
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                key_event @ KeyEvent {
                                    physical_key: PhysicalKey::Code(key_code),
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => {
                            // Vérifier si suffisamment de temps s'est écoulé depuis la dernière saisie
                            let now = Instant::now();
                            if now.duration_since(last_key_timestamp) < key_repeat_delay {
                                return; // Ignorer les répétitions trop rapides
                            }
                            last_key_timestamp = now;

                            match key_code {
                                KeyCode::Escape => {
                                    elwt.exit();
                                    return;
                                }
                                KeyCode::Enter if ui_state.input_active => {
                                    // Navigation par URL
                                    println!("Navigation vers URL: {}", ui_state.current_url);
                                    navigate_to_url(&mut runtime, &ui_state.current_url);
                                    ui_state.input_active = false;
                                    needs_redraw = true;
                                }
                                KeyCode::Backspace if ui_state.input_active => {
                                    // Gestion explicite de la touche Backspace pour la suppression
                                    if !ui_state.current_url.is_empty() {
                                        ui_state.current_url.pop();
                                        println!("URL après suppression: {}", ui_state.current_url);
                                        needs_redraw = true;
                                    }
                                }
                                _ if ui_state.input_active => {
                                    // Gestion de la saisie texte
                                    if let Some(text) = key_event.logical_key.to_text() {
                                        // Limiter la longueur maximale de l'URL
                                        if ui_state.current_url.len() < 50 {
                                            // Sanitiser l'entrée pour ne garder que les caractères valides
                                            let sanitized = sanitize_input(text);
                                            if !sanitized.is_empty() {
                                                ui_state.current_url.push_str(&sanitized);
                                                println!(
                                                    "URL mise à jour: {}",
                                                    ui_state.current_url
                                                );
                                                needs_redraw = true;
                                            }
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        WindowEvent::MouseInput {
                            state: ElementState::Pressed,
                            button: MouseButton::Left,
                            ..
                        } => {
                            // Gestion améliorée des clics
                            let (x, y) = cursor_position;

                            // Zone de la barre d'URL: toute la largeur, 30px de hauteur
                            if y < 30.0 {
                                ui_state.input_active = true;

                                // Si on clique près du début, on réinitialise l'URL
                                if x < 50.0 {
                                    ui_state.current_url = "unl://".to_string();
                                }

                                println!(
                                    "Barre d'URL activée. Tapez une adresse et appuyez sur Entrée."
                                );
                                needs_redraw = true;
                            } else {
                                if ui_state.input_active {
                                    ui_state.input_active = false;
                                    needs_redraw = true;
                                } else {
                                    // Clic dans la page
                                    handle_input(&event, &mut runtime);
                                    needs_redraw = true;
                                }
                            }
                        }
                        WindowEvent::RedrawRequested => {
                            if needs_redraw || last_render.elapsed() >= frame_rate_limit {
                                let frame = self.pixels.frame_mut();
                                // Passage d'une référence mutable à ui_state
                                render_page(&runtime, frame, 1024, &mut ui_state);
                                if let Err(err) = self.pixels.render() {
                                    println!("Erreur de rendu: {}", err);
                                    elwt.exit();
                                    return;
                                }
                                last_render = Instant::now();
                                needs_redraw = false;
                            }
                        }
                        _ => {}
                    },

                    Event::AboutToWait => {
                        // Demander un redraw seulement si nécessaire
                        if needs_redraw && last_render.elapsed() >= frame_rate_limit {
                            window.request_redraw();
                        }
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}
