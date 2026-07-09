use super::UnlIA;

#[derive(Debug, Clone)]
pub enum ElementType {
    Button,
    Text,
    Container,
    // Autres types possibles
}

impl UnlIA {
    pub fn generate_button(&mut self, id: &str, label: &str, action: &str) {
        let block = format!(
            "button \"{}\" at ({}, {}) action: {}",
            label,
            self.memory.layouts.get(id).unwrap_or(&(0.0, 0.0)).0,
            self.memory.layouts.get(id).unwrap_or(&(0.0, 0.0)).1,
            action
        );
        self.memory.builder_blocks.insert(id.to_string(), block);
    }

    pub fn generate_text(&mut self, id: &str, content: &str) {
        let block = format!(
            "text \"{}\" at ({}, {}) style: {}",
            content,
            self.memory.layouts.get(id).unwrap_or(&(0.0, 0.0)).0,
            self.memory.layouts.get(id).unwrap_or(&(0.0, 0.0)).1,
            self.memory.styles.get(id).unwrap_or(&"default".to_string())
        );
        self.memory.builder_blocks.insert(id.to_string(), block);
    }

    pub fn generate_container(&mut self, id: &str, width: f32, height: f32, children: &[&str]) {
        let mut block = format!(
            "container at ({}, {}) size({}, {}) style: {}\nchildren: [",
            self.memory.layouts.get(id).unwrap_or(&(0.0, 0.0)).0,
            self.memory.layouts.get(id).unwrap_or(&(0.0, 0.0)).1,
            width,
            height,
            self.memory.styles.get(id).unwrap_or(&"default".to_string())
        );

        for child in children {
            if let Some(child_block) = self.memory.builder_blocks.get(*child) {
                block.push_str(&format!("\n  {}", child_block));
            }
        }

        block.push_str("\n]");
        self.memory.builder_blocks.insert(id.to_string(), block);
    }

    // Fonction utilitaire pour créer rapidement un élément avec positionnement et style
    pub fn create_element(
        &mut self,
        element_type: ElementType,
        id: &str,
        params: &[(String, String)],
    ) {
        // Positionner l'élément
        if !self.memory.layouts.contains_key(id) {
            self.layout_element(id, 0.0, 0.0);
        }

        // Définir le style si nécessaire
        if !self.memory.styles.contains_key(id) {
            self.style_element(id, "default");
        }

        // Créer l'élément selon son type
        match element_type {
            ElementType::Button => {
                let label = params
                    .iter()
                    .find(|(k, _)| k == "label")
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("Button");
                let action = params
                    .iter()
                    .find(|(k, _)| k == "action")
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("noop");
                self.generate_button(id, label, action);
            }
            ElementType::Text => {
                let content = params
                    .iter()
                    .find(|(k, _)| k == "content")
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("");
                self.generate_text(id, content);
            }
            ElementType::Container => {
                let width = params
                    .iter()
                    .find(|(k, _)| k == "width")
                    .map(|(_, v)| v.parse::<f32>().unwrap_or(100.0))
                    .unwrap_or(100.0);
                let height = params
                    .iter()
                    .find(|(k, _)| k == "height")
                    .map(|(_, v)| v.parse::<f32>().unwrap_or(100.0))
                    .unwrap_or(100.0);
                let children_str = params
                    .iter()
                    .find(|(k, _)| k == "children")
                    .map(|(_, v)| v.as_str())
                    .unwrap_or("");
                let children: Vec<&str> = children_str.split(',').collect();
                self.generate_container(id, width, height, &children);
            }
        }
    }
}
