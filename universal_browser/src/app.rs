use crate::engine::engine_loop::Browser;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub fn launch_browser() {
    // Créer une fenêtre un peu plus grande pour avoir assez d'espace pour l'interface
    let width = 1024;
    let height = 768;

    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Universal Browser - UNL Navigator")
        .with_inner_size(LogicalSize::new(width as f64, height as f64))
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(width, height, &window);
    let pixels = Pixels::new(width, height, surface_texture).unwrap();

    let browser = Browser::new(pixels);

    println!("📱 Universal Browser démarré");
    println!("🔍 Instructions:");
    println!("  • Cliquez sur la barre d'URL en haut pour la modifier");
    println!("  • Tapez une adresse au format unl://nom_du_site");
    println!("  • Appuyez sur Entrée pour naviguer");
    println!("  • Exemple: unl://example");
    println!("  • Appuyez sur Échap pour quitter");
    println!("Note: La saisie est limitée à 50 caractères pour l'URL");

    browser.run(event_loop, window);
}
