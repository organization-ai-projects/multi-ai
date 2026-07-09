use crate::{
    export::ExportManager, glyph::GlyphManager, layout::LayoutManager, memory::Memory,
    style::StyleManager,
};

pub struct UnlIA {
    pub memory: Memory,
    glyph_manager: GlyphManager,
    export_manager: ExportManager,
    layout_manager: LayoutManager,
    style_manager: StyleManager,
}

impl UnlIA {
    pub fn new() -> Self {
        let memory = Memory::load();

        Self {
            memory: memory.clone(),
            glyph_manager: GlyphManager::new(memory.clone()),
            export_manager: ExportManager::new(),
            layout_manager: LayoutManager::new(),
            style_manager: StyleManager::new(),
        }
    }

    pub fn save(&self) {
        self.memory.save();
    }

    // Délégation des opérations de glyphes
    pub fn generate_glyph_variant(&mut self, c: char) {
        let variant = self.glyph_manager.create_variant(c);
        self.glyph_manager
            .add_variant_to_memory(&mut self.memory, c, variant);
    }

    pub fn choose_glyph_variant(&mut self, c: char, index: usize) {
        self.glyph_manager
            .choose_variant_in_memory(&mut self.memory, c, index);
    }

    pub fn list_variants(&self, c: char) {
        self.glyph_manager.list_variants_in_memory(&self.memory, c);
    }

    pub fn count_variants(&self, letter: char) -> usize {
        self.glyph_manager
            .count_variants_in_memory(&self.memory, letter)
    }

    pub fn generate_multiple_variants(&mut self, letter: char, count: usize) {
        self.glyph_manager
            .generate_multiple_variants_in_memory(&mut self.memory, letter, count);
    }

    pub fn clear_glyph_variants(&mut self, letter: char) {
        self.glyph_manager
            .clear_variants_in_memory(&mut self.memory, letter);
    }

    pub fn bitmap_to_string(&self, bitmap_data: &str) -> String {
        self.glyph_manager.bitmap_to_string(bitmap_data)
    }

    // Délégation des opérations de mise en page
    pub fn layout_element(&mut self, id: &str, x: f32, y: f32) {
        self.layout_manager
            .layout_element(&mut self.memory, id, x, y);
    }

    // Délégation des opérations de style
    pub fn style_element(&mut self, id: &str, style: &str) {
        self.style_manager
            .style_element(&mut self.memory, id, style);
    }

    // Délégation des opérations de génération d'éléments
    pub fn generate_text(&mut self, id: &str, content: &str) {
        let (x, y) = self.memory.layouts.get(id).copied().unwrap_or((0.0, 0.0));
        let style = self.memory.styles.get(id).cloned().unwrap_or_default();

        let block = format!("text \"{}\" at ({}, {}) style: {}", content, x, y, style);
        self.memory.builder_blocks.insert(id.to_string(), block);
    }

    // Méthodes d'export déléguées à l'ExportManager
    pub fn export_render_page(&self, path: &str) {
        self.export_manager.export_render_page(&self.memory, path);
    }

    pub fn export_all_for_renderer(&self, output_dir: &str) -> Result<(), String> {
        self.export_manager
            .export_all_for_renderer(&self.memory, output_dir)
    }
}
