use crate::memory::Memory;

pub struct StyleManager;

impl StyleManager {
    pub fn new() -> Self {
        Self {}
    }

    pub fn style_element(&self, memory: &mut Memory, id: &str, style: &str) {
        memory.styles.insert(id.to_string(), style.to_string());
    }

    // Autres méthodes de style...
}
