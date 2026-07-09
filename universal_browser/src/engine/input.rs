use crate::engine::router::navigate_to_url;
use crate::engine::runtime::Runtime;
use winit::event::{ElementState, MouseButton, WindowEvent};

pub fn handle_input(event: &WindowEvent, runtime: &mut Runtime) {
    if let WindowEvent::MouseInput {
        state: ElementState::Pressed,
        button: MouseButton::Left,
        ..
    } = event
    {
        // Parcourir les composants pour trouver le bouton cliqué
        for component in runtime.current_components() {
            if component.name == "Button" {
                // Vérifier si ce bouton a un événement associé
                if let Some((event_type, target_view)) = component.events.first() {
                    if event_type == "click" {
                        println!("Événement détecté: {} -> {}", event_type, target_view);

                        // Si le target_view commence par "view_", on enlève ce préfixe
                        let view_name = if target_view.starts_with("view_") {
                            &target_view[5..]
                        } else {
                            target_view
                        };

                        // Si le bouton contient une navigation externe (vers un autre site)
                        if let Some(content) = component
                            .properties
                            .iter()
                            .find(|(k, _)| k == "content")
                            .map(|(_, v)| v)
                        {
                            if let unilang::ast::Expr::String(label) = content {
                                if label.contains("example.unl") {
                                    navigate_to_url(runtime, "unl://example");
                                    println!("Navigation vers example.unl");
                                    return;
                                }
                            }
                        }

                        // Navigation interne (entre vues d'un même site)
                        runtime.set_view(view_name);
                        println!("Navigation interne vers la vue: {}", view_name);
                        return;
                    }
                }

                // Si aucun événement n'est défini, comportement par défaut selon le contenu
                if let Some((_, unilang::ast::Expr::String(label))) =
                    component.properties.iter().find(|(k, _)| k == "content")
                {
                    if label.contains("example.unl") {
                        navigate_to_url(runtime, "unl://example");
                        println!("Navigation par défaut vers example.unl");
                        return;
                    }
                }
            }
        }
    }
}
