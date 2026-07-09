use crate::memory::Memory;
use std::fs;
use std::path::Path;

pub struct ExportManager;

impl ExportManager {
    pub fn new() -> Self {
        Self {}
    }

    // Exporte les définitions des glyphes dans un format destiné à un moteur de rendu
    pub fn export_glyphs_for_renderer(&self, memory: &Memory, path: &str) -> Result<(), String> {
        let mut output = String::new();

        // En-tête explicatif
        output.push_str("// Définitions de glyphes pour moteur de rendu\n");
        output.push_str("// Format: [caractère] [type] [données]\n\n");

        // Exporter chaque glyphe
        for (char_str, glyph_data) in &memory.glyphs {
            if char_str.len() == 1 {
                // Format simplifié pour moteur de rendu externe
                output.push_str(&format!("{} {}\n", char_str, glyph_data));
            }
        }

        // Exporter les variantes si disponibles
        output.push_str("\n// Variantes de glyphes\n");
        for (c, knowledge) in &memory.glyph_knowledge {
            for (i, variant) in knowledge.variants.iter().enumerate() {
                output.push_str(&format!("{}_VAR{} VECTOR {}\n", *c, i, variant.vector));
                output.push_str(&format!("{}_VAR{} BITMAP {}\n", *c, i, variant.bitmap));

                if let Some(note) = &variant.notes {
                    output.push_str(&format!("{}_VAR{} NOTE {}\n", *c, i, note));
                }
            }

            // Indiquer la variante choisie
            if let Some(chosen) = knowledge.chosen_index {
                output.push_str(&format!("{} CHOSEN {}\n", *c, chosen));
            }
        }

        self.write_to_file(path, &output)
    }

    // Exporte les éléments de mise en page dans un format destiné à un moteur de rendu
    pub fn export_layout_for_renderer(&self, memory: &Memory, path: &str) -> Result<(), String> {
        let mut output = String::new();

        // En-tête explicatif
        output.push_str("// Définitions de mise en page pour moteur de rendu\n");
        output.push_str("// Format: [id] POSITION [x] [y]\n\n");

        // Exporter chaque position
        for (id, (x, y)) in &memory.layouts {
            output.push_str(&format!("{} POSITION {} {}\n", id, x, y));
        }

        self.write_to_file(path, &output)
    }

    // Exporte les styles dans un format destiné à un moteur de rendu
    pub fn export_styles_for_renderer(&self, memory: &Memory, path: &str) -> Result<(), String> {
        let mut output = String::new();

        // En-tête explicatif
        output.push_str("// Définitions de styles pour moteur de rendu\n");
        output.push_str("// Format: [id] STYLE [définition de style]\n\n");

        // Exporter chaque style
        for (id, style) in &memory.styles {
            output.push_str(&format!("{} STYLE {}\n", id, style));
        }

        self.write_to_file(path, &output)
    }

    // Exporte les éléments de construction dans un format destiné à un moteur de rendu
    pub fn export_builder_for_renderer(&self, memory: &Memory, path: &str) -> Result<(), String> {
        let mut output = String::new();

        // En-tête explicatif
        output.push_str("// Définitions d'éléments pour moteur de rendu\n");
        output.push_str("// Format: [id] ELEMENT [définition d'élément]\n\n");

        // Exporter chaque élément
        for (id, block) in &memory.builder_blocks {
            output.push_str(&format!("{} ELEMENT {}\n", id, block));
        }

        self.write_to_file(path, &output)
    }

    // Export complet pour moteur de rendu
    pub fn export_all_for_renderer(&self, memory: &Memory, output_dir: &str) -> Result<(), String> {
        // Créer le dossier de sortie
        fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;

        // Exporter chaque composant dans un fichier séparé
        self.export_glyphs_for_renderer(memory, &format!("{}/glyphs.def", output_dir))?;
        self.export_layout_for_renderer(memory, &format!("{}/layout.def", output_dir))?;
        self.export_styles_for_renderer(memory, &format!("{}/styles.def", output_dir))?;
        self.export_builder_for_renderer(memory, &format!("{}/elements.def", output_dir))?;

        // Créer un fichier d'index
        let mut index = String::new();
        index.push_str("// Index des fichiers de définition pour moteur de rendu\n");
        index.push_str("GLYPHS glyphs.def\n");
        index.push_str("LAYOUT layout.def\n");
        index.push_str("STYLES styles.def\n");
        index.push_str("ELEMENTS elements.def\n");

        self.write_to_file(&format!("{}/index.def", output_dir), &index)
    }

    // Export du rendu de page (format simplifié pour le nouveau format)
    pub fn export_render_page(&self, memory: &Memory, path: &str) {
        // Format spécifique pour le nouveau moteur de rendu
        let mut lines = Vec::new();

        for (id, block) in &memory.builder_blocks {
            lines.push(format!("{} => {}", id, block));
        }

        let export = lines.join("\n");

        // Créer le dossier parent si nécessaire
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).ok();
        }

        fs::write(path, export).ok();
    }

    // Fonction utilitaire pour écrire dans un fichier avec création du dossier parent
    fn write_to_file(&self, path: &str, content: &str) -> Result<(), String> {
        // Créer le dossier parent si nécessaire
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        fs::write(path, content).map_err(|e| e.to_string())
    }
}
