use crate::evolution::Evolution;
use crate::species::{Lineage, SpeciesType};
use std::fs;
use std::path::Path;

const AI_TEMPLATE: &str = include_str!("templates/ai_project/src/main.rs");
const MEMORY_TEMPLATE: &str = include_str!("templates/ai_project/src/memory.rs");
const LOGIC_TEMPLATE: &str = include_str!("templates/ai_project/src/logic.rs");
const NEURON_TEMPLATE: &str = include_str!("templates/ai_project/src/neuron.rs");
const CARGO_TEMPLATE: &str = include_str!("templates/ai_project/Cargo.toml");

#[derive(Clone)]
pub struct ProjectManager {
    base_dir: String,
}

impl ProjectManager {
    pub fn new(base_dir: String) -> Self {
        Self { base_dir }
    }

    pub fn create_project(&self, id: usize, species: Option<SpeciesType>) -> std::io::Result<()> {
        let ai_dir = format!("{}/ai_{:05}", self.base_dir, id);
        let src_dir = format!("{}/src", ai_dir);
        fs::create_dir_all(&src_dir)?;

        self.create_project_files(&ai_dir, id)?;
        self.create_lineage(&ai_dir, id, species)?;

        Ok(())
    }

    fn create_project_files(&self, ai_dir: &str, id: usize) -> std::io::Result<()> {
        let cargo_content = CARGO_TEMPLATE.replace("{{id}}", &format!("{:05}", id));
        fs::write(format!("{}/Cargo.toml", ai_dir), cargo_content)?;

        fs::write(format!("{}/src/main.rs", ai_dir), AI_TEMPLATE)?;
        fs::write(format!("{}/src/memory.rs", ai_dir), MEMORY_TEMPLATE)?;
        fs::write(format!("{}/src/logic.rs", ai_dir), LOGIC_TEMPLATE)?;
        fs::write(format!("{}/src/neuron.rs", ai_dir), NEURON_TEMPLATE)?;

        Ok(())
    }

    fn create_lineage(
        &self,
        ai_dir: &str,
        id: usize,
        species: Option<SpeciesType>,
    ) -> std::io::Result<()> {
        let lineage = Lineage {
            species: species.unwrap_or_else(SpeciesType::random),
            ancestor_id: format!("ai_{:05}", id),
            generation_created: 0,
            mutations_survived: 0,
        };

        fs::write(
            format!("{}/lineage.ron", ai_dir),
            ron::to_string(&lineage).unwrap(),
        )
    }

    pub fn create_evolved_project(
        &self,
        source_path: &str,
        id: usize,
        evolution: &mut Evolution,
    ) -> std::io::Result<()> {
        let content = fs::read_to_string(source_path)?;
        self.create_project(id, None)?;

        let new_path = format!("{}/ai_{:05}/src/main.rs", self.base_dir, id);
        fs::write(new_path, evolution.mutate_code(&content))?;

        Ok(())
    }
}
