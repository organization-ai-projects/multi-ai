use std::fs;
use std::path::Path;

pub struct ProjectTemplates {
    pub ai_template: String,
    pub memory_template: String,
    pub logic_template: String,
    pub neuron_template: String,
    pub cargo_template: String,
}

impl ProjectTemplates {
    pub fn new() -> Self {
        Self {
            ai_template: include_str!("../../templates/ai_project/src/main.rs").to_string(),
            memory_template: include_str!("../../templates/ai_project/src/memory.rs").to_string(),
            logic_template: include_str!("../../templates/ai_project/src/logic.rs").to_string(),
            neuron_template: include_str!("../../templates/ai_project/src/neuron.rs").to_string(),
            cargo_template: include_str!("../../templates/ai_project/Cargo.toml").to_string(),
        }
    }

    pub fn write_to_project(&self, project_dir: &Path, id: usize) -> std::io::Result<()> {
        let src_dir = project_dir.join("src");
        fs::create_dir_all(&src_dir)?;

        let cargo_content = self.cargo_template.replace("{{id}}", &format!("{:05}", id));
        fs::write(project_dir.join("Cargo.toml"), cargo_content)?;
        fs::write(src_dir.join("main.rs"), &self.ai_template)?;
        fs::write(src_dir.join("memory.rs"), &self.memory_template)?;
        fs::write(src_dir.join("logic.rs"), &self.logic_template)?;
        fs::write(src_dir.join("neuron.rs"), &self.neuron_template)?;
        Ok(())
    }
}
