use crate::memory::Memory;

#[derive(Debug, Clone, Copy)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

pub struct LayoutManager;

impl LayoutManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn layout_element(&self, memory: &mut Memory, id: &str, x: f32, y: f32) {
        memory.layouts.insert(id.to_string(), (x, y));
    }

    // Positionnement automatique avec alignement
    pub fn auto_layout(
        &mut self,
        id: &str,
        h_align: HorizontalAlign,
        v_align: VerticalAlign,
        container_width: f32,
        container_height: f32,
    ) {
        let x = match h_align {
            HorizontalAlign::Left => 0.0,
            HorizontalAlign::Center => container_width / 2.0,
            HorizontalAlign::Right => container_width,
        };

        let y = match v_align {
            VerticalAlign::Top => 0.0,
            VerticalAlign::Middle => container_height / 2.0,
            VerticalAlign::Bottom => container_height,
        };

        self.memory.layouts.insert(id.to_string(), (x, y));
    }

    // Positionnement relatif à un autre élément
    pub fn relative_layout(&mut self, id: &str, relative_to: &str, offset_x: f32, offset_y: f32) {
        if let Some(&(base_x, base_y)) = self.memory.layouts.get(relative_to) {
            self.memory
                .layouts
                .insert(id.to_string(), (base_x + offset_x, base_y + offset_y));
        }
    }

    // Organisation en grille
    pub fn grid_layout(&mut self, ids: &[&str], cols: usize, cell_width: f32, cell_height: f32) {
        for (i, &id) in ids.iter().enumerate() {
            let row = (i / cols) as f32;
            let col = (i % cols) as f32;

            let x = col * cell_width;
            let y = row * cell_height;

            self.memory.layouts.insert(id.to_string(), (x, y));
        }
    }
}
