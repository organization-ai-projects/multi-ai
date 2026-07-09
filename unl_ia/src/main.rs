use std::env;
use unl_ia::UnlIA;

fn main() {
    // Initialiser l'IA
    let mut ia = UnlIA::new();

    // Récupérer les arguments
    let args: Vec<String> = env::args().collect();
    let reset = args.iter().any(|arg| arg == "--reset" || arg == "-r");

    // Traitement des glyphes pour le moteur de rendu
    if reset {
        println!("Nettoyage des variantes existantes du glyphe 'A'");
        ia.clear_glyph_variants('A');
    }

    let count = ia.count_variants('A');
    if count < 4 {
        println!("Génération de variantes pour le glyphe 'A'");
        ia.generate_multiple_variants('A', 4 - count);
    }

    // Afficher les variantes disponibles
    ia.list_variants('A');

    // Visualiser chaque variante
    if let Some(glyph) = ia.memory.glyph_knowledge.get(&'A') {
        for (i, variant) in glyph.variants.iter().enumerate() {
            println!("\nVisualisation de la variante {}:", i);
            println!("{}", ia.bitmap_to_string(&variant.bitmap));
        }
    }

    // Choisir une variante
    let variant_index = args
        .iter()
        .position(|arg| arg == "--select" || arg == "-s")
        .and_then(|pos| {
            if pos + 1 < args.len() {
                args[pos + 1].parse::<usize>().ok()
            } else {
                None
            }
        })
        .unwrap_or(0);

    ia.choose_glyph_variant('A', variant_index);
    println!("Variante choisie : index {}", variant_index);

    // Créer un exemple simple de mise en page
    ia.layout_element("titre", 100.0, 50.0);
    ia.style_element("titre", "font-size: 24px; font-weight: bold;");
    ia.generate_text("titre", "ABCDEF");

    // Sauvegarder l'état de l'IA
    ia.save();

    // Exporter pour le moteur de rendu
    let output_dir = "unl_ia/output";
    let renderer_dir = "unl_ia/renderer_output";

    // Créer les dossiers de sortie
    std::fs::create_dir_all(output_dir).unwrap_or_else(|e| {
        println!("Avertissement: Impossible de créer le dossier: {}", e);
    });

    // Exporter la page
    let page_path = format!("{}/page.ron", output_dir);
    ia.export_render_page(&page_path);
    println!("Page exportée: {}", page_path);

    // Exporter les définitions pour le moteur de rendu
    match ia.export_all_for_renderer(renderer_dir) {
        Ok(_) => println!(
            "Définitions pour moteur de rendu exportées: {}",
            renderer_dir
        ),
        Err(e) => println!("Erreur lors de l'export: {}", e),
    }
}
